/* FP:source_map.rs-0001 */ // Types for tracking pieces of source code within a crate.
/* FP:source_map.rs-0002 */ //
/* FP:source_map.rs-0003 */ // The [`SourceMap`] tracks all the source code used within a single crate, mapping
/* FP:source_map.rs-0004 */ // from integer byte positions to the original source code location. Each bit
/* FP:source_map.rs-0005 */ // of source parsed during crate parsing (typically files, in-memory strings,
/* FP:source_map.rs-0006 */ // or various bits of macro expansion) cover a continuous range of bytes in the
/* FP:source_map.rs-0007 */ // `SourceMap` and are represented by [`SourceFile`]s. Byte positions are stored in
/* FP:source_map.rs-0008 */ // [`Span`] and used pervasively in the compiler. They are absolute positions
/* FP:source_map.rs-0009 */ // within the `SourceMap`, which upon request can be converted to line and column
/* FP:source_map.rs-0010 */ // information, source code snippets, etc.
/* FP:source_map.rs-0011 */ 
/* FP:source_map.rs-0012 */ use std::fs::File;
/* FP:source_map.rs-0013 */ use std::io::{self, BorrowedBuf, Read};
/* FP:source_map.rs-0014 */ use std::{fs, path};
/* FP:source_map.rs-0015 */ 
/* FP:source_map.rs-0016 */ use crate::rustc_data_structures::sync::{IntoDynSyncSend, MappedReadGuard, ReadGuard, RwLock};
/* FP:source_map.rs-0017 */ use crate::rustc_data_structures::unhash::UnhashMap;
/* FP:source_map.rs-0018 */ use rustc_macros::{Decodable, Encodable};
/* FP:source_map.rs-0019 */ use tracing::{debug, instrument, trace};
/* FP:source_map.rs-0020 */ 
/* FP:source_map.rs-0021 */ use crate::*;
/* FP:source_map.rs-0022 */ 
/* FP:source_map.rs-0023 */ #[cfg(test)]
/* FP:source_map.rs-0025 */ 
/* FP:source_map.rs-0026 */ /// Returns the span itself if it doesn't come from a macro expansion,
/* FP:source_map.rs-0027 */ /// otherwise return the call site span up to the `enclosing_sp` by
/* FP:source_map.rs-0028 */ /// following the `expn_data` chain.
/* FP:source_map.rs-0029 */ pub fn original_sp(sp: Span, enclosing_sp: Span) -> Span {
/* FP:source_map.rs-0030 */     let ctxt = sp.ctxt();
/* FP:source_map.rs-0031 */     if ctxt.is_root() {
/* FP:source_map.rs-0032 */         return sp;
/* FP:source_map.rs-0033 */     }
/* FP:source_map.rs-0034 */ 
/* FP:source_map.rs-0035 */     let enclosing_ctxt = enclosing_sp.ctxt();
/* FP:source_map.rs-0036 */     let expn_data1 = ctxt.outer_expn_data();
/* FP:source_map.rs-0037 */     if !enclosing_ctxt.is_root()
/* FP:source_map.rs-0038 */         && expn_data1.call_site == enclosing_ctxt.outer_expn_data().call_site
/* FP:source_map.rs-0039 */     {
/* FP:source_map.rs-0040 */         sp
/* FP:source_map.rs-0041 */     } else {
/* FP:source_map.rs-0042 */         original_sp(expn_data1.call_site, enclosing_sp)
/* FP:source_map.rs-0043 */     }
/* FP:source_map.rs-0044 */ }
/* FP:source_map.rs-0045 */ 
/* FP:source_map.rs-0046 */ mod monotonic {
/* FP:source_map.rs-0047 */     use std::ops::{Deref, DerefMut};
/* FP:source_map.rs-0048 */ 
/* FP:source_map.rs-0049 */     /// A `MonotonicVec` is a `Vec` which can only be grown.
/* FP:source_map.rs-0050 */     /// Once inserted, an element can never be removed or swapped,
/* FP:source_map.rs-0051 */     /// guaranteeing that any indices into a `MonotonicVec` are stable
/* FP:source_map.rs-0052 */     // This is declared in its own module to ensure that the private
/* FP:source_map.rs-0053 */     // field is inaccessible
/* FP:source_map.rs-0054 */     pub struct MonotonicVec<T>(Vec<T>);
/* FP:source_map.rs-0055 */     impl<T> MonotonicVec<T> {
/* FP:source_map.rs-0056 */         pub(super) fn push(&mut self, val: T) {
/* FP:source_map.rs-0057 */             self.0.push(val);
/* FP:source_map.rs-0058 */         }
/* FP:source_map.rs-0059 */     }
/* FP:source_map.rs-0060 */ 
/* FP:source_map.rs-0061 */     impl<T> Default for MonotonicVec<T> {
/* FP:source_map.rs-0062 */         fn default() -> Self {
/* FP:source_map.rs-0063 */             MonotonicVec(vec![])
/* FP:source_map.rs-0064 */         }
/* FP:source_map.rs-0065 */     }
/* FP:source_map.rs-0066 */ 
/* FP:source_map.rs-0067 */     impl<T> Deref for MonotonicVec<T> {
/* FP:source_map.rs-0068 */         type Target = Vec<T>;
/* FP:source_map.rs-0069 */         fn deref(&self) -> &Self::Target {
/* FP:source_map.rs-0070 */             &self.0
/* FP:source_map.rs-0071 */         }
/* FP:source_map.rs-0072 */     }
/* FP:source_map.rs-0073 */ 
/* FP:source_map.rs-0074 */     impl<T> !DerefMut for MonotonicVec<T> {}
/* FP:source_map.rs-0075 */ }
/* FP:source_map.rs-0076 */ 
/* FP:source_map.rs-0077 */ #[derive(Clone, Encodable, Decodable, Debug, Copy, PartialEq, Hash, HashStable_Generic)]
/* FP:source_map.rs-0078 */ pub struct Spanned<T> {
/* FP:source_map.rs-0079 */     pub node: T,
/* FP:source_map.rs-0080 */     pub span: Span,
/* FP:source_map.rs-0081 */ }
/* FP:source_map.rs-0082 */ 
/* FP:source_map.rs-0083 */ pub fn respan<T>(sp: Span, t: T) -> Spanned<T> {
/* FP:source_map.rs-0084 */     Spanned { node: t, span: sp }
/* FP:source_map.rs-0085 */ }
/* FP:source_map.rs-0086 */ 
/* FP:source_map.rs-0087 */ pub fn dummy_spanned<T>(t: T) -> Spanned<T> {
/* FP:source_map.rs-0088 */     respan(DUMMY_SP, t)
/* FP:source_map.rs-0089 */ }
/* FP:source_map.rs-0090 */ 
/* FP:source_map.rs-0091 */ // _____________________________________________________________________________
/* FP:source_map.rs-0092 */ // SourceFile, MultiByteChar, FileName, FileLines
/* FP:source_map.rs-0093 */ //
/* FP:source_map.rs-0094 */ 
/* FP:source_map.rs-0095 */ /// An abstraction over the fs operations used by the Parser.
/* FP:source_map.rs-0096 */ pub trait FileLoader {
/* FP:source_map.rs-0097 */     /// Query the existence of a file.
/* FP:source_map.rs-0098 */     fn file_exists(&self, path: &Path) -> bool;
/* FP:source_map.rs-0099 */ 
/* FP:source_map.rs-0100 */     /// Read the contents of a UTF-8 file into memory.
/* FP:source_map.rs-0101 */     /// This function must return a String because we normalize
/* FP:source_map.rs-0102 */     /// source files, which may require resizing.
/* FP:source_map.rs-0103 */     fn read_file(&self, path: &Path) -> io::Result<String>;
/* FP:source_map.rs-0104 */ 
/* FP:source_map.rs-0105 */     /// Read the contents of a potentially non-UTF-8 file into memory.
/* FP:source_map.rs-0106 */     /// We don't normalize binary files, so we can start in an Arc.
/* FP:source_map.rs-0107 */     fn read_binary_file(&self, path: &Path) -> io::Result<Arc<[u8]>>;
/* FP:source_map.rs-0108 */ }
/* FP:source_map.rs-0109 */ 
/* FP:source_map.rs-0110 */ /// A FileLoader that uses std::fs to load real files.
/* FP:source_map.rs-0111 */ pub struct RealFileLoader;
/* FP:source_map.rs-0112 */ 
/* FP:source_map.rs-0113 */ impl FileLoader for RealFileLoader {
/* FP:source_map.rs-0114 */     fn file_exists(&self, path: &Path) -> bool {
/* FP:source_map.rs-0115 */         path.exists()
/* FP:source_map.rs-0116 */     }
/* FP:source_map.rs-0117 */ 
/* FP:source_map.rs-0118 */     fn read_file(&self, path: &Path) -> io::Result<String> {
/* FP:source_map.rs-0119 */         let mut file = File::open(path)?;
/* FP:source_map.rs-0120 */         let size = file.metadata().map(|metadata| metadata.len()).ok().unwrap_or(0);
/* FP:source_map.rs-0121 */ 
/* FP:source_map.rs-0122 */         if size > SourceFile::MAX_FILE_SIZE.into() {
/* FP:source_map.rs-0123 */             return Err(io::Error::other(format!(
/* FP:source_map.rs-0124 */                 "text files larger than {} bytes are unsupported",
/* FP:source_map.rs-0125 */                 SourceFile::MAX_FILE_SIZE
/* FP:source_map.rs-0126 */             )));
/* FP:source_map.rs-0127 */         }
/* FP:source_map.rs-0128 */         let mut contents = String::new();
/* FP:source_map.rs-0129 */         file.read_to_string(&mut contents)?;
/* FP:source_map.rs-0130 */         Ok(contents)
/* FP:source_map.rs-0131 */     }
/* FP:source_map.rs-0132 */ 
/* FP:source_map.rs-0133 */     fn read_binary_file(&self, path: &Path) -> io::Result<Arc<[u8]>> {
/* FP:source_map.rs-0134 */         let mut file = fs::File::open(path)?;
/* FP:source_map.rs-0135 */         let len = file.metadata()?.len();
/* FP:source_map.rs-0136 */ 
/* FP:source_map.rs-0137 */         let mut bytes = Arc::new_uninit_slice(len as usize);
/* FP:source_map.rs-0138 */         let mut buf = BorrowedBuf::from(Arc::get_mut(&mut bytes).unwrap());
/* FP:source_map.rs-0139 */         match file.read_buf_exact(buf.unfilled()) {
/* FP:source_map.rs-0140 */             Ok(()) => {}
/* FP:source_map.rs-0141 */             Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
/* FP:source_map.rs-0142 */                 drop(bytes);
/* FP:source_map.rs-0143 */                 return fs::read(path).map(Vec::into);
/* FP:source_map.rs-0144 */             }
/* FP:source_map.rs-0145 */             Err(e) => return Err(e),
/* FP:source_map.rs-0146 */         }
/* FP:source_map.rs-0147 */         // SAFETY: If the read_buf_exact call returns Ok(()), then we have
/* FP:source_map.rs-0148 */         // read len bytes and initialized the buffer.
/* FP:source_map.rs-0149 */         let bytes = unsafe { bytes.assume_init() };
/* FP:source_map.rs-0150 */ 
/* FP:source_map.rs-0151 */         // At this point, we've read all the bytes that filesystem metadata reported exist.
/* FP:source_map.rs-0152 */         // But we are not guaranteed to be at the end of the file, because we did not attempt to do
/* FP:source_map.rs-0153 */         // a read with a non-zero-sized buffer and get Ok(0).
/* FP:source_map.rs-0154 */         // So we do small read to a fixed-size buffer. If the read returns no bytes then we're
/* FP:source_map.rs-0155 */         // already done, and we just return the Arc we built above.
/* FP:source_map.rs-0156 */         // If the read returns bytes however, we just fall back to reading into a Vec then turning
/* FP:source_map.rs-0157 */         // that into an Arc, losing our nice peak memory behavior. This fallback code path should
/* FP:source_map.rs-0158 */         // be rarely exercised.
/* FP:source_map.rs-0159 */ 
/* FP:source_map.rs-0160 */         let mut probe = [0u8; 32];
/* FP:source_map.rs-0161 */         let n = loop {
/* FP:source_map.rs-0162 */             match file.read(&mut probe) {
/* FP:source_map.rs-0163 */                 Ok(0) => return Ok(bytes),
/* FP:source_map.rs-0164 */                 Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
/* FP:source_map.rs-0165 */                 Err(e) => return Err(e),
/* FP:source_map.rs-0166 */                 Ok(n) => break n,
/* FP:source_map.rs-0167 */             }
/* FP:source_map.rs-0168 */         };
/* FP:source_map.rs-0169 */         let mut bytes: Vec<u8> = bytes.iter().copied().chain(probe[..n].iter().copied()).collect();
/* FP:source_map.rs-0170 */         file.read_to_end(&mut bytes)?;
/* FP:source_map.rs-0171 */         Ok(bytes.into())
/* FP:source_map.rs-0172 */     }
/* FP:source_map.rs-0173 */ }
/* FP:source_map.rs-0174 */ 
/* FP:source_map.rs-0175 */ // _____________________________________________________________________________
/* FP:source_map.rs-0176 */ // SourceMap
/* FP:source_map.rs-0177 */ //
/* FP:source_map.rs-0178 */ 
/* FP:source_map.rs-0179 */ #[derive(Default)]
/* FP:source_map.rs-0180 */ struct SourceMapFiles {
/* FP:source_map.rs-0181 */     source_files: monotonic::MonotonicVec<Arc<SourceFile>>,
/* FP:source_map.rs-0182 */     stable_id_to_source_file: UnhashMap<StableSourceFileId, Arc<SourceFile>>,
/* FP:source_map.rs-0183 */ }
/* FP:source_map.rs-0184 */ 
/* FP:source_map.rs-0185 */ /// Used to construct a `SourceMap` with `SourceMap::with_inputs`.
/* FP:source_map.rs-0186 */ pub struct SourceMapInputs {
/* FP:source_map.rs-0187 */     pub file_loader: Box<dyn FileLoader + Send + Sync>,
/* FP:source_map.rs-0188 */     pub path_mapping: FilePathMapping,
/* FP:source_map.rs-0189 */     pub hash_kind: SourceFileHashAlgorithm,
/* FP:source_map.rs-0190 */     pub checksum_hash_kind: Option<SourceFileHashAlgorithm>,
/* FP:source_map.rs-0191 */ }
/* FP:source_map.rs-0192 */ 
/* FP:source_map.rs-0193 */ pub struct SourceMap {
/* FP:source_map.rs-0194 */     files: RwLock<SourceMapFiles>,
/* FP:source_map.rs-0195 */     file_loader: IntoDynSyncSend<Box<dyn FileLoader + Sync + Send>>,
/* FP:source_map.rs-0196 */ 
/* FP:source_map.rs-0197 */     // This is used to apply the file path remapping as specified via
/* FP:source_map.rs-0198 */     // `--remap-path-prefix` to all `SourceFile`s allocated within this `SourceMap`.
/* FP:source_map.rs-0199 */     path_mapping: FilePathMapping,
/* FP:source_map.rs-0200 */ 
/* FP:source_map.rs-0201 */     /// The algorithm used for hashing the contents of each source file.
/* FP:source_map.rs-0202 */     hash_kind: SourceFileHashAlgorithm,
/* FP:source_map.rs-0203 */ 
/* FP:source_map.rs-0204 */     /// Similar to `hash_kind`, however this algorithm is used for checksums to determine if a crate is fresh.
/* FP:source_map.rs-0205 */     /// `cargo` is the primary user of these.
/* FP:source_map.rs-0206 */     ///
/* FP:source_map.rs-0207 */     /// If this is equal to `hash_kind` then the checksum won't be computed twice.
/* FP:source_map.rs-0208 */     checksum_hash_kind: Option<SourceFileHashAlgorithm>,
/* FP:source_map.rs-0209 */ }
/* FP:source_map.rs-0210 */ 
/* FP:source_map.rs-0211 */ impl SourceMap {
/* FP:source_map.rs-0212 */     pub fn new(path_mapping: FilePathMapping) -> SourceMap {
/* FP:source_map.rs-0213 */         Self::with_inputs(SourceMapInputs {
/* FP:source_map.rs-0214 */             file_loader: Box::new(RealFileLoader),
/* FP:source_map.rs-0215 */             path_mapping,
/* FP:source_map.rs-0216 */             hash_kind: SourceFileHashAlgorithm::Md5,
/* FP:source_map.rs-0217 */             checksum_hash_kind: None,
/* FP:source_map.rs-0218 */         })
/* FP:source_map.rs-0219 */     }
/* FP:source_map.rs-0220 */ 
/* FP:source_map.rs-0221 */     pub fn with_inputs(
/* FP:source_map.rs-0222 */         SourceMapInputs { file_loader, path_mapping, hash_kind, checksum_hash_kind }: SourceMapInputs,
/* FP:source_map.rs-0223 */     ) -> SourceMap {
/* FP:source_map.rs-0224 */         SourceMap {
/* FP:source_map.rs-0225 */             files: Default::default(),
/* FP:source_map.rs-0226 */             file_loader: IntoDynSyncSend(file_loader),
/* FP:source_map.rs-0227 */             path_mapping,
/* FP:source_map.rs-0228 */             hash_kind,
/* FP:source_map.rs-0229 */             checksum_hash_kind,
/* FP:source_map.rs-0230 */         }
/* FP:source_map.rs-0231 */     }
/* FP:source_map.rs-0232 */ 
/* FP:source_map.rs-0233 */     pub fn path_mapping(&self) -> &FilePathMapping {
/* FP:source_map.rs-0234 */         &self.path_mapping
/* FP:source_map.rs-0235 */     }
/* FP:source_map.rs-0236 */ 
/* FP:source_map.rs-0237 */     pub fn file_exists(&self, path: &Path) -> bool {
/* FP:source_map.rs-0238 */         self.file_loader.file_exists(path)
/* FP:source_map.rs-0239 */     }
/* FP:source_map.rs-0240 */ 
/* FP:source_map.rs-0241 */     pub fn load_file(&self, path: &Path) -> io::Result<Arc<SourceFile>> {
/* FP:source_map.rs-0242 */         let src = self.file_loader.read_file(path)?;
/* FP:source_map.rs-0243 */         let filename = path.to_owned().into();
/* FP:source_map.rs-0244 */         Ok(self.new_source_file(filename, src))
/* FP:source_map.rs-0245 */     }
/* FP:source_map.rs-0246 */ 
/* FP:source_map.rs-0247 */     /// Loads source file as a binary blob.
/* FP:source_map.rs-0248 */     ///
/* FP:source_map.rs-0249 */     /// Unlike `load_file`, guarantees that no normalization like BOM-removal
/* FP:source_map.rs-0250 */     /// takes place.
/* FP:source_map.rs-0251 */     pub fn load_binary_file(&self, path: &Path) -> io::Result<(Arc<[u8]>, Span)> {
/* FP:source_map.rs-0252 */         let bytes = self.file_loader.read_binary_file(path)?;
/* FP:source_map.rs-0253 */ 
/* FP:source_map.rs-0254 */         // We need to add file to the `SourceMap`, so that it is present
/* FP:source_map.rs-0255 */         // in dep-info. There's also an edge case that file might be both
/* FP:source_map.rs-0256 */         // loaded as a binary via `include_bytes!` and as proper `SourceFile`
/* FP:source_map.rs-0257 */         // via `mod`, so we try to use real file contents and not just an
/* FP:source_map.rs-0258 */         // empty string.
/* FP:source_map.rs-0259 */         let text = std::str::from_utf8(&bytes).unwrap_or("").to_string();
/* FP:source_map.rs-0260 */         let file = self.new_source_file(path.to_owned().into(), text);
/* FP:source_map.rs-0261 */         Ok((
/* FP:source_map.rs-0262 */             bytes,
/* FP:source_map.rs-0263 */             Span::new(
/* FP:source_map.rs-0264 */                 file.start_pos,
/* FP:source_map.rs-0265 */                 BytePos(file.start_pos.0 + file.source_len.0),
/* FP:source_map.rs-0266 */                 SyntaxContext::root(),
/* FP:source_map.rs-0267 */                 None,
/* FP:source_map.rs-0268 */             ),
/* FP:source_map.rs-0269 */         ))
/* FP:source_map.rs-0270 */     }
/* FP:source_map.rs-0271 */ 
/* FP:source_map.rs-0272 */     // By returning a `MonotonicVec`, we ensure that consumers cannot invalidate
/* FP:source_map.rs-0273 */     // any existing indices pointing into `files`.
/* FP:source_map.rs-0274 */     pub fn files(&self) -> MappedReadGuard<'_, monotonic::MonotonicVec<Arc<SourceFile>>> {
/* FP:source_map.rs-0275 */         ReadGuard::map(self.files.borrow(), |files| &files.source_files)
/* FP:source_map.rs-0276 */     }
/* FP:source_map.rs-0277 */ 
/* FP:source_map.rs-0278 */     pub fn source_file_by_stable_id(
/* FP:source_map.rs-0279 */         &self,
/* FP:source_map.rs-0280 */         stable_id: StableSourceFileId,
/* FP:source_map.rs-0281 */     ) -> Option<Arc<SourceFile>> {
/* FP:source_map.rs-0282 */         self.files.borrow().stable_id_to_source_file.get(&stable_id).cloned()
/* FP:source_map.rs-0283 */     }
/* FP:source_map.rs-0284 */ 
/* FP:source_map.rs-0285 */     fn register_source_file(
/* FP:source_map.rs-0286 */         &self,
/* FP:source_map.rs-0287 */         file_id: StableSourceFileId,
/* FP:source_map.rs-0288 */         mut file: SourceFile,
/* FP:source_map.rs-0289 */     ) -> Result<Arc<SourceFile>, OffsetOverflowError> {
/* FP:source_map.rs-0290 */         let mut files = self.files.borrow_mut();
/* FP:source_map.rs-0291 */ 
/* FP:source_map.rs-0292 */         file.start_pos = BytePos(if let Some(last_file) = files.source_files.last() {
/* FP:source_map.rs-0293 */             // Add one so there is some space between files. This lets us distinguish
/* FP:source_map.rs-0294 */             // positions in the `SourceMap`, even in the presence of zero-length files.
/* FP:source_map.rs-0295 */             last_file.end_position().0.checked_add(1).ok_or(OffsetOverflowError)?
/* FP:source_map.rs-0296 */         } else {
/* FP:source_map.rs-0297 */             0
/* FP:source_map.rs-0298 */         });
/* FP:source_map.rs-0299 */ 
/* FP:source_map.rs-0300 */         let file = Arc::new(file);
/* FP:source_map.rs-0301 */         files.source_files.push(Arc::clone(&file));
/* FP:source_map.rs-0302 */         files.stable_id_to_source_file.insert(file_id, Arc::clone(&file));
/* FP:source_map.rs-0303 */ 
/* FP:source_map.rs-0304 */         Ok(file)
/* FP:source_map.rs-0305 */     }
/* FP:source_map.rs-0306 */ 
/* FP:source_map.rs-0307 */     /// Creates a new `SourceFile`.
/* FP:source_map.rs-0308 */     /// If a file already exists in the `SourceMap` with the same ID, that file is returned
/* FP:source_map.rs-0309 */     /// unmodified.
/* FP:source_map.rs-0310 */     pub fn new_source_file(&self, filename: FileName, src: String) -> Arc<SourceFile> {
/* FP:source_map.rs-0311 */         self.try_new_source_file(filename, src).unwrap_or_else(|OffsetOverflowError| {
/* FP:source_map.rs-0312 */             eprintln!(
/* FP:source_map.rs-0313 */                 "fatal error: rustc does not support text files larger than {} bytes",
/* FP:source_map.rs-0314 */                 SourceFile::MAX_FILE_SIZE
/* FP:source_map.rs-0315 */             );
/* FP:source_map.rs-0316 */             crate::fatal_error::FatalError.raise()
/* FP:source_map.rs-0317 */         })
/* FP:source_map.rs-0318 */     }
/* FP:source_map.rs-0319 */ 
/* FP:source_map.rs-0320 */     fn try_new_source_file(
/* FP:source_map.rs-0321 */         &self,
/* FP:source_map.rs-0322 */         filename: FileName,
/* FP:source_map.rs-0323 */         src: String,
/* FP:source_map.rs-0324 */     ) -> Result<Arc<SourceFile>, OffsetOverflowError> {
/* FP:source_map.rs-0325 */         // Note that filename may not be a valid path, eg it may be `<anon>` etc,
/* FP:source_map.rs-0326 */         // but this is okay because the directory determined by `path.pop()` will
/* FP:source_map.rs-0327 */         // be empty, so the working directory will be used.
/* FP:source_map.rs-0328 */         let (filename, _) = self.path_mapping.map_filename_prefix(&filename);
/* FP:source_map.rs-0329 */ 
/* FP:source_map.rs-0330 */         let stable_id = StableSourceFileId::from_filename_in_current_crate(&filename);
/* FP:source_map.rs-0331 */         match self.source_file_by_stable_id(stable_id) {
/* FP:source_map.rs-0332 */             Some(lrc_sf) => Ok(lrc_sf),
/* FP:source_map.rs-0333 */             None => {
/* FP:source_map.rs-0334 */                 let source_file =
/* FP:source_map.rs-0335 */                     SourceFile::new(filename, src, self.hash_kind, self.checksum_hash_kind)?;
/* FP:source_map.rs-0336 */ 
/* FP:source_map.rs-0337 */                 // Let's make sure the file_id we generated above actually matches
/* FP:source_map.rs-0338 */                 // the ID we generate for the SourceFile we just created.
/* FP:source_map.rs-0339 */                 debug_assert_eq!(source_file.stable_id, stable_id);
/* FP:source_map.rs-0340 */ 
/* FP:source_map.rs-0341 */                 self.register_source_file(stable_id, source_file)
/* FP:source_map.rs-0342 */             }
/* FP:source_map.rs-0343 */         }
/* FP:source_map.rs-0344 */     }
/* FP:source_map.rs-0345 */ 
/* FP:source_map.rs-0346 */     /// Allocates a new `SourceFile` representing a source file from an external
/* FP:source_map.rs-0347 */     /// crate. The source code of such an "imported `SourceFile`" is not available,
/* FP:source_map.rs-0348 */     /// but we still know enough to generate accurate debuginfo location
/* FP:source_map.rs-0349 */     /// information for things inlined from other crates.
/* FP:source_map.rs-0350 */     pub fn new_imported_source_file(
/* FP:source_map.rs-0351 */         &self,
/* FP:source_map.rs-0352 */         filename: FileName,
/* FP:source_map.rs-0353 */         src_hash: SourceFileHash,
/* FP:source_map.rs-0354 */         checksum_hash: Option<SourceFileHash>,
/* FP:source_map.rs-0355 */         stable_id: StableSourceFileId,
/* FP:source_map.rs-0356 */         source_len: u32,
/* FP:source_map.rs-0357 */         cnum: CrateNum,
/* FP:source_map.rs-0358 */         file_local_lines: FreezeLock<SourceFileLines>,
/* FP:source_map.rs-0359 */         multibyte_chars: Vec<MultiByteChar>,
/* FP:source_map.rs-0360 */         normalized_pos: Vec<NormalizedPos>,
/* FP:source_map.rs-0361 */         metadata_index: u32,
/* FP:source_map.rs-0362 */     ) -> Arc<SourceFile> {
/* FP:source_map.rs-0363 */         let source_len = RelativeBytePos::from_u32(source_len);
/* FP:source_map.rs-0364 */ 
/* FP:source_map.rs-0365 */         let source_file = SourceFile {
/* FP:source_map.rs-0366 */             name: filename,
/* FP:source_map.rs-0367 */             src: None,
/* FP:source_map.rs-0368 */             src_hash,
/* FP:source_map.rs-0369 */             checksum_hash,
/* FP:source_map.rs-0370 */             external_src: FreezeLock::new(ExternalSource::Foreign {
/* FP:source_map.rs-0371 */                 kind: ExternalSourceKind::AbsentOk,
/* FP:source_map.rs-0372 */                 metadata_index,
/* FP:source_map.rs-0373 */             }),
/* FP:source_map.rs-0374 */             start_pos: BytePos(0),
/* FP:source_map.rs-0375 */             source_len,
/* FP:source_map.rs-0376 */             lines: file_local_lines,
/* FP:source_map.rs-0377 */             multibyte_chars,
/* FP:source_map.rs-0378 */             normalized_pos,
/* FP:source_map.rs-0379 */             stable_id,
/* FP:source_map.rs-0380 */             cnum,
/* FP:source_map.rs-0381 */         };
/* FP:source_map.rs-0382 */ 
/* FP:source_map.rs-0383 */         self.register_source_file(stable_id, source_file)
/* FP:source_map.rs-0384 */             .expect("not enough address space for imported source file")
/* FP:source_map.rs-0385 */     }
/* FP:source_map.rs-0386 */ 
/* FP:source_map.rs-0387 */     /// If there is a doctest offset, applies it to the line.
/* FP:source_map.rs-0388 */     pub fn doctest_offset_line(&self, file: &FileName, orig: usize) -> usize {
/* FP:source_map.rs-0389 */         match file {
/* FP:source_map.rs-0390 */             FileName::DocTest(_, offset) => {
/* FP:source_map.rs-0391 */                 if *offset < 0 {
/* FP:source_map.rs-0392 */                     orig - (-(*offset)) as usize
/* FP:source_map.rs-0393 */                 } else {
/* FP:source_map.rs-0394 */                     orig + *offset as usize
/* FP:source_map.rs-0395 */                 }
/* FP:source_map.rs-0396 */             }
/* FP:source_map.rs-0397 */             _ => orig,
/* FP:source_map.rs-0398 */         }
/* FP:source_map.rs-0399 */     }
/* FP:source_map.rs-0400 */ 
/* FP:source_map.rs-0401 */     /// Return the SourceFile that contains the given `BytePos`
/* FP:source_map.rs-0402 */     pub fn lookup_source_file(&self, pos: BytePos) -> Arc<SourceFile> {
/* FP:source_map.rs-0403 */         let idx = self.lookup_source_file_idx(pos);
/* FP:source_map.rs-0404 */         Arc::clone(&(*self.files.borrow().source_files)[idx])
/* FP:source_map.rs-0405 */     }
/* FP:source_map.rs-0406 */ 
/* FP:source_map.rs-0407 */     /// Looks up source information about a `BytePos`.
/* FP:source_map.rs-0408 */     pub fn lookup_char_pos(&self, pos: BytePos) -> Loc {
/* FP:source_map.rs-0409 */         let sf = self.lookup_source_file(pos);
/* FP:source_map.rs-0410 */         let (line, col, col_display) = sf.lookup_file_pos_with_col_display(pos);
/* FP:source_map.rs-0411 */         Loc { file: sf, line, col, col_display }
/* FP:source_map.rs-0412 */     }
/* FP:source_map.rs-0413 */ 
/* FP:source_map.rs-0414 */     /// If the corresponding `SourceFile` is empty, does not return a line number.
/* FP:source_map.rs-0415 */     pub fn lookup_line(&self, pos: BytePos) -> Result<SourceFileAndLine, Arc<SourceFile>> {
/* FP:source_map.rs-0416 */         let f = self.lookup_source_file(pos);
/* FP:source_map.rs-0417 */ 
/* FP:source_map.rs-0418 */         let pos = f.relative_position(pos);
/* FP:source_map.rs-0419 */         match f.lookup_line(pos) {
/* FP:source_map.rs-0420 */             Some(line) => Ok(SourceFileAndLine { sf: f, line }),
/* FP:source_map.rs-0421 */             None => Err(f),
/* FP:source_map.rs-0422 */         }
/* FP:source_map.rs-0423 */     }
/* FP:source_map.rs-0424 */ 
/* FP:source_map.rs-0425 */     pub fn span_to_string(
/* FP:source_map.rs-0426 */         &self,
/* FP:source_map.rs-0427 */         sp: Span,
/* FP:source_map.rs-0428 */         filename_display_pref: FileNameDisplayPreference,
/* FP:source_map.rs-0429 */     ) -> String {
/* FP:source_map.rs-0430 */         let (source_file, lo_line, lo_col, hi_line, hi_col) = self.span_to_location_info(sp);
/* FP:source_map.rs-0431 */ 
/* FP:source_map.rs-0432 */         let file_name = match source_file {
/* FP:source_map.rs-0433 */             Some(sf) => sf.name.display(filename_display_pref).to_string(),
/* FP:source_map.rs-0434 */             None => return "no-location".to_string(),
/* FP:source_map.rs-0435 */         };
/* FP:source_map.rs-0436 */ 
/* FP:source_map.rs-0437 */         format!(
/* FP:source_map.rs-0438 */             "{file_name}:{lo_line}:{lo_col}{}",
/* FP:source_map.rs-0439 */             if let FileNameDisplayPreference::Short = filename_display_pref {
/* FP:source_map.rs-0440 */                 String::new()
/* FP:source_map.rs-0441 */             } else {
/* FP:source_map.rs-0442 */                 format!(": {hi_line}:{hi_col}")
/* FP:source_map.rs-0443 */             }
/* FP:source_map.rs-0444 */         )
/* FP:source_map.rs-0445 */     }
/* FP:source_map.rs-0446 */ 
/* FP:source_map.rs-0447 */     pub fn span_to_location_info(
/* FP:source_map.rs-0448 */         &self,
/* FP:source_map.rs-0449 */         sp: Span,
/* FP:source_map.rs-0450 */     ) -> (Option<Arc<SourceFile>>, usize, usize, usize, usize) {
/* FP:source_map.rs-0451 */         if self.files.borrow().source_files.is_empty() || sp.is_dummy() {
/* FP:source_map.rs-0452 */             return (None, 0, 0, 0, 0);
/* FP:source_map.rs-0453 */         }
/* FP:source_map.rs-0454 */ 
/* FP:source_map.rs-0455 */         let lo = self.lookup_char_pos(sp.lo());
/* FP:source_map.rs-0456 */         let hi = self.lookup_char_pos(sp.hi());
/* FP:source_map.rs-0457 */         (Some(lo.file), lo.line, lo.col.to_usize() + 1, hi.line, hi.col.to_usize() + 1)
/* FP:source_map.rs-0458 */     }
/* FP:source_map.rs-0459 */ 
/* FP:source_map.rs-0460 */     /// Format the span location suitable for embedding in build artifacts
/* FP:source_map.rs-0461 */     pub fn span_to_embeddable_string(&self, sp: Span) -> String {
/* FP:source_map.rs-0462 */         self.span_to_string(sp, FileNameDisplayPreference::Remapped)
/* FP:source_map.rs-0463 */     }
/* FP:source_map.rs-0464 */ 
/* FP:source_map.rs-0465 */     /// Format the span location to be printed in diagnostics. Must not be emitted
/* FP:source_map.rs-0466 */     /// to build artifacts as this may leak local file paths. Use span_to_embeddable_string
/* FP:source_map.rs-0467 */     /// for string suitable for embedding.
/* FP:source_map.rs-0468 */     pub fn span_to_diagnostic_string(&self, sp: Span) -> String {
/* FP:source_map.rs-0469 */         self.span_to_string(sp, self.path_mapping.filename_display_for_diagnostics)
/* FP:source_map.rs-0470 */     }
/* FP:source_map.rs-0471 */ 
/* FP:source_map.rs-0472 */     pub fn span_to_filename(&self, sp: Span) -> FileName {
/* FP:source_map.rs-0473 */         self.lookup_char_pos(sp.lo()).file.name.clone()
/* FP:source_map.rs-0474 */     }
/* FP:source_map.rs-0475 */ 
/* FP:source_map.rs-0476 */     pub fn filename_for_diagnostics<'a>(&self, filename: &'a FileName) -> FileNameDisplay<'a> {
/* FP:source_map.rs-0477 */         filename.display(self.path_mapping.filename_display_for_diagnostics)
/* FP:source_map.rs-0478 */     }
/* FP:source_map.rs-0479 */ 
/* FP:source_map.rs-0480 */     pub fn is_multiline(&self, sp: Span) -> bool {
/* FP:source_map.rs-0481 */         let lo = self.lookup_source_file_idx(sp.lo());
/* FP:source_map.rs-0482 */         let hi = self.lookup_source_file_idx(sp.hi());
/* FP:source_map.rs-0483 */         if lo != hi {
/* FP:source_map.rs-0484 */             return true;
/* FP:source_map.rs-0485 */         }
/* FP:source_map.rs-0486 */         let f = Arc::clone(&(*self.files.borrow().source_files)[lo]);
/* FP:source_map.rs-0487 */         let lo = f.relative_position(sp.lo());
/* FP:source_map.rs-0488 */         let hi = f.relative_position(sp.hi());
/* FP:source_map.rs-0489 */         f.lookup_line(lo) != f.lookup_line(hi)
/* FP:source_map.rs-0490 */     }
/* FP:source_map.rs-0491 */ 
/* FP:source_map.rs-0492 */     #[instrument(skip(self), level = "trace")]
/* FP:source_map.rs-0493 */     pub fn is_valid_span(&self, sp: Span) -> Result<(Loc, Loc), SpanLinesError> {
/* FP:source_map.rs-0494 */         let lo = self.lookup_char_pos(sp.lo());
/* FP:source_map.rs-0495 */         trace!(?lo);
/* FP:source_map.rs-0496 */         let hi = self.lookup_char_pos(sp.hi());
/* FP:source_map.rs-0497 */         trace!(?hi);
/* FP:source_map.rs-0498 */         if lo.file.start_pos != hi.file.start_pos {
/* FP:source_map.rs-0499 */             return Err(SpanLinesError::DistinctSources(Box::new(DistinctSources {
/* FP:source_map.rs-0500 */                 begin: (lo.file.name.clone(), lo.file.start_pos),
/* FP:source_map.rs-0501 */                 end: (hi.file.name.clone(), hi.file.start_pos),
/* FP:source_map.rs-0502 */             })));
/* FP:source_map.rs-0503 */         }
/* FP:source_map.rs-0504 */         Ok((lo, hi))
/* FP:source_map.rs-0505 */     }
/* FP:source_map.rs-0506 */ 
/* FP:source_map.rs-0507 */     pub fn is_line_before_span_empty(&self, sp: Span) -> bool {
/* FP:source_map.rs-0508 */         match self.span_to_prev_source(sp) {
/* FP:source_map.rs-0509 */             Ok(s) => s.rsplit_once('\n').unwrap_or(("", &s)).1.trim_start().is_empty(),
/* FP:source_map.rs-0510 */             Err(_) => false,
/* FP:source_map.rs-0511 */         }
/* FP:source_map.rs-0512 */     }
/* FP:source_map.rs-0513 */ 
/* FP:source_map.rs-0514 */     pub fn span_to_lines(&self, sp: Span) -> FileLinesResult {
/* FP:source_map.rs-0515 */         debug!("span_to_lines(sp={:?})", sp);
/* FP:source_map.rs-0516 */         let (lo, hi) = self.is_valid_span(sp)?;
/* FP:source_map.rs-0517 */         assert!(hi.line >= lo.line);
/* FP:source_map.rs-0518 */ 
/* FP:source_map.rs-0519 */         if sp.is_dummy() {
/* FP:source_map.rs-0520 */             return Ok(FileLines { file: lo.file, lines: Vec::new() });
/* FP:source_map.rs-0521 */         }
/* FP:source_map.rs-0522 */ 
/* FP:source_map.rs-0523 */         let mut lines = Vec::with_capacity(hi.line - lo.line + 1);
/* FP:source_map.rs-0524 */ 
/* FP:source_map.rs-0525 */         // The span starts partway through the first line,
/* FP:source_map.rs-0526 */         // but after that it starts from offset 0.
/* FP:source_map.rs-0527 */         let mut start_col = lo.col;
/* FP:source_map.rs-0528 */ 
/* FP:source_map.rs-0529 */         // For every line but the last, it extends from `start_col`
/* FP:source_map.rs-0530 */         // and to the end of the line. Be careful because the line
/* FP:source_map.rs-0531 */         // numbers in Loc are 1-based, so we subtract 1 to get 0-based
/* FP:source_map.rs-0532 */         // lines.
/* FP:source_map.rs-0533 */         //
/* FP:source_map.rs-0534 */         // FIXME: now that we handle DUMMY_SP up above, we should consider
/* FP:source_map.rs-0535 */         // asserting that the line numbers here are all indeed 1-based.
/* FP:source_map.rs-0536 */         let hi_line = hi.line.saturating_sub(1);
/* FP:source_map.rs-0537 */         for line_index in lo.line.saturating_sub(1)..hi_line {
/* FP:source_map.rs-0538 */             let line_len = lo.file.get_line(line_index).map_or(0, |s| s.chars().count());
/* FP:source_map.rs-0539 */             lines.push(LineInfo { line_index, start_col, end_col: CharPos::from_usize(line_len) });
/* FP:source_map.rs-0540 */             start_col = CharPos::from_usize(0);
/* FP:source_map.rs-0541 */         }
/* FP:source_map.rs-0542 */ 
/* FP:source_map.rs-0543 */         // For the last line, it extends from `start_col` to `hi.col`:
/* FP:source_map.rs-0544 */         lines.push(LineInfo { line_index: hi_line, start_col, end_col: hi.col });
/* FP:source_map.rs-0545 */ 
/* FP:source_map.rs-0546 */         Ok(FileLines { file: lo.file, lines })
/* FP:source_map.rs-0547 */     }
/* FP:source_map.rs-0548 */ 
/* FP:source_map.rs-0549 */     /// Extracts the source surrounding the given `Span` using the `extract_source` function. The
/* FP:source_map.rs-0550 */     /// extract function takes three arguments: a string slice containing the source, an index in
/* FP:source_map.rs-0551 */     /// the slice for the beginning of the span and an index in the slice for the end of the span.
/* FP:source_map.rs-0552 */     pub fn span_to_source<F, T>(&self, sp: Span, extract_source: F) -> Result<T, SpanSnippetError>
/* FP:source_map.rs-0553 */     where
/* FP:source_map.rs-0554 */         F: Fn(&str, usize, usize) -> Result<T, SpanSnippetError>,
/* FP:source_map.rs-0555 */     {
/* FP:source_map.rs-0556 */         let local_begin = self.lookup_byte_offset(sp.lo());
/* FP:source_map.rs-0557 */         let local_end = self.lookup_byte_offset(sp.hi());
/* FP:source_map.rs-0558 */ 
/* FP:source_map.rs-0559 */         if local_begin.sf.start_pos != local_end.sf.start_pos {
/* FP:source_map.rs-0560 */             Err(SpanSnippetError::DistinctSources(Box::new(DistinctSources {
/* FP:source_map.rs-0561 */                 begin: (local_begin.sf.name.clone(), local_begin.sf.start_pos),
/* FP:source_map.rs-0562 */                 end: (local_end.sf.name.clone(), local_end.sf.start_pos),
/* FP:source_map.rs-0563 */             })))
/* FP:source_map.rs-0564 */         } else {
/* FP:source_map.rs-0565 */             self.ensure_source_file_source_present(&local_begin.sf);
/* FP:source_map.rs-0566 */ 
/* FP:source_map.rs-0567 */             let start_index = local_begin.pos.to_usize();
/* FP:source_map.rs-0568 */             let end_index = local_end.pos.to_usize();
/* FP:source_map.rs-0569 */             let source_len = local_begin.sf.source_len.to_usize();
/* FP:source_map.rs-0570 */ 
/* FP:source_map.rs-0571 */             if start_index > end_index || end_index > source_len {
/* FP:source_map.rs-0572 */                 return Err(SpanSnippetError::MalformedForSourcemap(MalformedSourceMapPositions {
/* FP:source_map.rs-0573 */                     name: local_begin.sf.name.clone(),
/* FP:source_map.rs-0574 */                     source_len,
/* FP:source_map.rs-0575 */                     begin_pos: local_begin.pos,
/* FP:source_map.rs-0576 */                     end_pos: local_end.pos,
/* FP:source_map.rs-0577 */                 }));
/* FP:source_map.rs-0578 */             }
/* FP:source_map.rs-0579 */ 
/* FP:source_map.rs-0580 */             if let Some(ref src) = local_begin.sf.src {
/* FP:source_map.rs-0581 */                 extract_source(src, start_index, end_index)
/* FP:source_map.rs-0582 */             } else if let Some(src) = local_begin.sf.external_src.read().get_source() {
/* FP:source_map.rs-0583 */                 extract_source(src, start_index, end_index)
/* FP:source_map.rs-0584 */             } else {
/* FP:source_map.rs-0585 */                 Err(SpanSnippetError::SourceNotAvailable { filename: local_begin.sf.name.clone() })
/* FP:source_map.rs-0586 */             }
/* FP:source_map.rs-0587 */         }
/* FP:source_map.rs-0588 */     }
/* FP:source_map.rs-0589 */ 
/* FP:source_map.rs-0590 */     pub fn is_span_accessible(&self, sp: Span) -> bool {
/* FP:source_map.rs-0591 */         self.span_to_source(sp, |src, start_index, end_index| {
/* FP:source_map.rs-0592 */             Ok(src.get(start_index..end_index).is_some())
/* FP:source_map.rs-0593 */         })
/* FP:source_map.rs-0594 */         .is_ok_and(|is_accessible| is_accessible)
/* FP:source_map.rs-0595 */     }
/* FP:source_map.rs-0596 */ 
/* FP:source_map.rs-0597 */     /// Returns the source snippet as `String` corresponding to the given `Span`.
/* FP:source_map.rs-0598 */     pub fn span_to_snippet(&self, sp: Span) -> Result<String, SpanSnippetError> {
/* FP:source_map.rs-0599 */         self.span_to_source(sp, |src, start_index, end_index| {
/* FP:source_map.rs-0600 */             src.get(start_index..end_index)
/* FP:source_map.rs-0601 */                 .map(|s| s.to_string())
/* FP:source_map.rs-0602 */                 .ok_or(SpanSnippetError::IllFormedSpan(sp))
/* FP:source_map.rs-0603 */         })
/* FP:source_map.rs-0604 */     }
/* FP:source_map.rs-0605 */ 
/* FP:source_map.rs-0606 */     pub fn span_to_margin(&self, sp: Span) -> Option<usize> {
/* FP:source_map.rs-0607 */         Some(self.indentation_before(sp)?.len())
/* FP:source_map.rs-0608 */     }
/* FP:source_map.rs-0609 */ 
/* FP:source_map.rs-0610 */     pub fn indentation_before(&self, sp: Span) -> Option<String> {
/* FP:source_map.rs-0611 */         self.span_to_source(sp, |src, start_index, _| {
/* FP:source_map.rs-0612 */             let before = &src[..start_index];
/* FP:source_map.rs-0613 */             let last_line = before.rsplit_once('\n').map_or(before, |(_, last)| last);
/* FP:source_map.rs-0614 */             Ok(last_line
/* FP:source_map.rs-0615 */                 .split_once(|c: char| !c.is_whitespace())
/* FP:source_map.rs-0616 */                 .map_or(last_line, |(indent, _)| indent)
/* FP:source_map.rs-0617 */                 .to_string())
/* FP:source_map.rs-0618 */         })
/* FP:source_map.rs-0619 */         .ok()
/* FP:source_map.rs-0620 */     }
/* FP:source_map.rs-0621 */ 
/* FP:source_map.rs-0622 */     /// Returns the source snippet as `String` before the given `Span`.
/* FP:source_map.rs-0623 */     pub fn span_to_prev_source(&self, sp: Span) -> Result<String, SpanSnippetError> {
/* FP:source_map.rs-0624 */         self.span_to_source(sp, |src, start_index, _| {
/* FP:source_map.rs-0625 */             src.get(..start_index).map(|s| s.to_string()).ok_or(SpanSnippetError::IllFormedSpan(sp))
/* FP:source_map.rs-0626 */         })
/* FP:source_map.rs-0627 */     }
/* FP:source_map.rs-0628 */ 
/* FP:source_map.rs-0629 */     /// Extends the given `Span` to just after the previous occurrence of `c`. Return the same span
/* FP:source_map.rs-0630 */     /// if no character could be found or if an error occurred while retrieving the code snippet.
/* FP:source_map.rs-0631 */     pub fn span_extend_to_prev_char(&self, sp: Span, c: char, accept_newlines: bool) -> Span {
/* FP:source_map.rs-0632 */         if let Ok(prev_source) = self.span_to_prev_source(sp) {
/* FP:source_map.rs-0633 */             let prev_source = prev_source.rsplit(c).next().unwrap_or("");
/* FP:source_map.rs-0634 */             if !prev_source.is_empty() && (accept_newlines || !prev_source.contains('\n')) {
/* FP:source_map.rs-0635 */                 return sp.with_lo(BytePos(sp.lo().0 - prev_source.len() as u32));
/* FP:source_map.rs-0636 */             }
/* FP:source_map.rs-0637 */         }
/* FP:source_map.rs-0638 */ 
/* FP:source_map.rs-0639 */         sp
/* FP:source_map.rs-0640 */     }
/* FP:source_map.rs-0641 */ 
/* FP:source_map.rs-0642 */     /// Extends the given `Span` to just before the previous occurrence of `c`. Return the same span
/* FP:source_map.rs-0643 */     /// if an error occurred while retrieving the code snippet.
/* FP:source_map.rs-0644 */     pub fn span_extend_to_prev_char_before(
/* FP:source_map.rs-0645 */         &self,
/* FP:source_map.rs-0646 */         sp: Span,
/* FP:source_map.rs-0647 */         c: char,
/* FP:source_map.rs-0648 */         accept_newlines: bool,
/* FP:source_map.rs-0649 */     ) -> Span {
/* FP:source_map.rs-0650 */         if let Ok(prev_source) = self.span_to_prev_source(sp) {
/* FP:source_map.rs-0651 */             let prev_source = prev_source.rsplit(c).next().unwrap_or("");
/* FP:source_map.rs-0652 */             if accept_newlines || !prev_source.contains('\n') {
/* FP:source_map.rs-0653 */                 return sp.with_lo(BytePos(sp.lo().0 - prev_source.len() as u32 - 1_u32));
/* FP:source_map.rs-0654 */             }
/* FP:source_map.rs-0655 */         }
/* FP:source_map.rs-0656 */ 
/* FP:source_map.rs-0657 */         sp
/* FP:source_map.rs-0658 */     }
/* FP:source_map.rs-0659 */ 
/* FP:source_map.rs-0660 */     /// Extends the given `Span` to just after the previous occurrence of `pat` when surrounded by
/* FP:source_map.rs-0661 */     /// whitespace. Returns None if the pattern could not be found or if an error occurred while
/* FP:source_map.rs-0662 */     /// retrieving the code snippet.
/* FP:source_map.rs-0663 */     pub fn span_extend_to_prev_str(
/* FP:source_map.rs-0664 */         &self,
/* FP:source_map.rs-0665 */         sp: Span,
/* FP:source_map.rs-0666 */         pat: &str,
/* FP:source_map.rs-0667 */         accept_newlines: bool,
/* FP:source_map.rs-0668 */         include_whitespace: bool,
/* FP:source_map.rs-0669 */     ) -> Option<Span> {
/* FP:source_map.rs-0670 */         // assure that the pattern is delimited, to avoid the following
/* FP:source_map.rs-0671 */         //     fn my_fn()
/* FP:source_map.rs-0672 */         //           ^^^^ returned span without the check
/* FP:source_map.rs-0673 */         //     ---------- correct span
/* FP:source_map.rs-0674 */         let prev_source = self.span_to_prev_source(sp).ok()?;
/* FP:source_map.rs-0675 */         for ws in &[" ", "\t", "\n"] {
/* FP:source_map.rs-0676 */             let pat = pat.to_owned() + ws;
/* FP:source_map.rs-0677 */             if let Some(pat_pos) = prev_source.rfind(&pat) {
/* FP:source_map.rs-0678 */                 let just_after_pat_pos = pat_pos + pat.len() - 1;
/* FP:source_map.rs-0679 */                 let just_after_pat_plus_ws = if include_whitespace {
/* FP:source_map.rs-0680 */                     just_after_pat_pos
/* FP:source_map.rs-0681 */                         + prev_source[just_after_pat_pos..]
/* FP:source_map.rs-0682 */                             .find(|c: char| !c.is_whitespace())
/* FP:source_map.rs-0683 */                             .unwrap_or(0)
/* FP:source_map.rs-0684 */                 } else {
/* FP:source_map.rs-0685 */                     just_after_pat_pos
/* FP:source_map.rs-0686 */                 };
/* FP:source_map.rs-0687 */                 let len = prev_source.len() - just_after_pat_plus_ws;
/* FP:source_map.rs-0688 */                 let prev_source = &prev_source[just_after_pat_plus_ws..];
/* FP:source_map.rs-0689 */                 if accept_newlines || !prev_source.trim_start().contains('\n') {
/* FP:source_map.rs-0690 */                     return Some(sp.with_lo(BytePos(sp.lo().0 - len as u32)));
/* FP:source_map.rs-0691 */                 }
/* FP:source_map.rs-0692 */             }
/* FP:source_map.rs-0693 */         }
/* FP:source_map.rs-0694 */ 
/* FP:source_map.rs-0695 */         None
/* FP:source_map.rs-0696 */     }
/* FP:source_map.rs-0697 */ 
/* FP:source_map.rs-0698 */     /// Returns the source snippet as `String` after the given `Span`.
/* FP:source_map.rs-0699 */     pub fn span_to_next_source(&self, sp: Span) -> Result<String, SpanSnippetError> {
/* FP:source_map.rs-0700 */         self.span_to_source(sp, |src, _, end_index| {
/* FP:source_map.rs-0701 */             src.get(end_index..).map(|s| s.to_string()).ok_or(SpanSnippetError::IllFormedSpan(sp))
/* FP:source_map.rs-0702 */         })
/* FP:source_map.rs-0703 */     }
/* FP:source_map.rs-0704 */ 
/* FP:source_map.rs-0705 */     /// Extends the given `Span` while the next character matches the predicate
/* FP:source_map.rs-0706 */     pub fn span_extend_while(
/* FP:source_map.rs-0707 */         &self,
/* FP:source_map.rs-0708 */         span: Span,
/* FP:source_map.rs-0709 */         f: impl Fn(char) -> bool,
/* FP:source_map.rs-0710 */     ) -> Result<Span, SpanSnippetError> {
/* FP:source_map.rs-0711 */         self.span_to_source(span, |s, _start, end| {
/* FP:source_map.rs-0712 */             let n = s[end..].char_indices().find(|&(_, c)| !f(c)).map_or(s.len() - end, |(i, _)| i);
/* FP:source_map.rs-0713 */             Ok(span.with_hi(span.hi() + BytePos(n as u32)))
/* FP:source_map.rs-0714 */         })
/* FP:source_map.rs-0715 */     }
/* FP:source_map.rs-0716 */ 
/* FP:source_map.rs-0717 */     /// Extends the span to include any trailing whitespace, or returns the original
/* FP:source_map.rs-0718 */     /// span if a `SpanSnippetError` was encountered.
/* FP:source_map.rs-0719 */     pub fn span_extend_while_whitespace(&self, span: Span) -> Span {
/* FP:source_map.rs-0720 */         self.span_extend_while(span, char::is_whitespace).unwrap_or(span)
/* FP:source_map.rs-0721 */     }
/* FP:source_map.rs-0722 */ 
/* FP:source_map.rs-0723 */     /// Extends the given `Span` to previous character while the previous character matches the predicate
/* FP:source_map.rs-0724 */     pub fn span_extend_prev_while(
/* FP:source_map.rs-0725 */         &self,
/* FP:source_map.rs-0726 */         span: Span,
/* FP:source_map.rs-0727 */         f: impl Fn(char) -> bool,
/* FP:source_map.rs-0728 */     ) -> Result<Span, SpanSnippetError> {
/* FP:source_map.rs-0729 */         self.span_to_source(span, |s, start, _end| {
/* FP:source_map.rs-0730 */             let n = s[..start]
/* FP:source_map.rs-0731 */                 .char_indices()
/* FP:source_map.rs-0732 */                 .rfind(|&(_, c)| !f(c))
/* FP:source_map.rs-0733 */                 .map_or(start, |(i, _)| start - i - 1);
/* FP:source_map.rs-0734 */             Ok(span.with_lo(span.lo() - BytePos(n as u32)))
/* FP:source_map.rs-0735 */         })
/* FP:source_map.rs-0736 */     }
/* FP:source_map.rs-0737 */ 
/* FP:source_map.rs-0738 */     /// Extends the given `Span` to just before the next occurrence of `c`.
/* FP:source_map.rs-0739 */     pub fn span_extend_to_next_char(&self, sp: Span, c: char, accept_newlines: bool) -> Span {
/* FP:source_map.rs-0740 */         if let Ok(next_source) = self.span_to_next_source(sp) {
/* FP:source_map.rs-0741 */             let next_source = next_source.split(c).next().unwrap_or("");
/* FP:source_map.rs-0742 */             if !next_source.is_empty() && (accept_newlines || !next_source.contains('\n')) {
/* FP:source_map.rs-0743 */                 return sp.with_hi(BytePos(sp.hi().0 + next_source.len() as u32));
/* FP:source_map.rs-0744 */             }
/* FP:source_map.rs-0745 */         }
/* FP:source_map.rs-0746 */ 
/* FP:source_map.rs-0747 */         sp
/* FP:source_map.rs-0748 */     }
/* FP:source_map.rs-0749 */ 
/* FP:source_map.rs-0750 */     /// Extends the given `Span` to contain the entire line it is on.
/* FP:source_map.rs-0751 */     pub fn span_extend_to_line(&self, sp: Span) -> Span {
/* FP:source_map.rs-0752 */         self.span_extend_to_prev_char(self.span_extend_to_next_char(sp, '\n', true), '\n', true)
/* FP:source_map.rs-0753 */     }
/* FP:source_map.rs-0754 */ 
/* FP:source_map.rs-0755 */     /// Given a `Span`, tries to get a shorter span ending before the first occurrence of `char`
/* FP:source_map.rs-0756 */     /// `c`.
/* FP:source_map.rs-0757 */     pub fn span_until_char(&self, sp: Span, c: char) -> Span {
/* FP:source_map.rs-0758 */         match self.span_to_snippet(sp) {
/* FP:source_map.rs-0759 */             Ok(snippet) => {
/* FP:source_map.rs-0760 */                 let snippet = snippet.split(c).next().unwrap_or("").trim_end();
/* FP:source_map.rs-0761 */                 if !snippet.is_empty() && !snippet.contains('\n') {
/* FP:source_map.rs-0762 */                     sp.with_hi(BytePos(sp.lo().0 + snippet.len() as u32))
/* FP:source_map.rs-0763 */                 } else {
/* FP:source_map.rs-0764 */                     sp
/* FP:source_map.rs-0765 */                 }
/* FP:source_map.rs-0766 */             }
/* FP:source_map.rs-0767 */             _ => sp,
/* FP:source_map.rs-0768 */         }
/* FP:source_map.rs-0769 */     }
/* FP:source_map.rs-0770 */ 
/* FP:source_map.rs-0771 */     /// Given a 'Span', tries to tell if it's wrapped by "<>" or "()"
/* FP:source_map.rs-0772 */     /// the algorithm searches if the next character is '>' or ')' after skipping white space
/* FP:source_map.rs-0773 */     /// then searches the previous character to match '<' or '(' after skipping white space
/* FP:source_map.rs-0774 */     /// return true if wrapped by '<>' or '()'
/* FP:source_map.rs-0775 */     pub fn span_wrapped_by_angle_or_parentheses(&self, span: Span) -> bool {
/* FP:source_map.rs-0776 */         self.span_to_source(span, |src, start_index, end_index| {
/* FP:source_map.rs-0777 */             if src.get(start_index..end_index).is_none() {
/* FP:source_map.rs-0778 */                 return Ok(false);
/* FP:source_map.rs-0779 */             }
/* FP:source_map.rs-0780 */             // test the right side to match '>' after skipping white space
/* FP:source_map.rs-0781 */             let end_src = &src[end_index..];
/* FP:source_map.rs-0782 */             let mut i = 0;
/* FP:source_map.rs-0783 */             let mut found_right_parentheses = false;
/* FP:source_map.rs-0784 */             let mut found_right_angle = false;
/* FP:source_map.rs-0785 */             while let Some(cc) = end_src.chars().nth(i) {
/* FP:source_map.rs-0786 */                 if cc == ' ' {
/* FP:source_map.rs-0787 */                     i = i + 1;
/* FP:source_map.rs-0788 */                 } else if cc == '>' {
/* FP:source_map.rs-0789 */                     // found > in the right;
/* FP:source_map.rs-0790 */                     found_right_angle = true;
/* FP:source_map.rs-0791 */                     break;
/* FP:source_map.rs-0792 */                 } else if cc == ')' {
/* FP:source_map.rs-0793 */                     found_right_parentheses = true;
/* FP:source_map.rs-0794 */                     break;
/* FP:source_map.rs-0795 */                 } else {
/* FP:source_map.rs-0796 */                     // failed to find '>' return false immediately
/* FP:source_map.rs-0797 */                     return Ok(false);
/* FP:source_map.rs-0798 */                 }
/* FP:source_map.rs-0799 */             }
/* FP:source_map.rs-0800 */             // test the left side to match '<' after skipping white space
/* FP:source_map.rs-0801 */             i = start_index;
/* FP:source_map.rs-0802 */             let start_src = &src[0..start_index];
/* FP:source_map.rs-0803 */             while let Some(cc) = start_src.chars().nth(i) {
/* FP:source_map.rs-0804 */                 if cc == ' ' {
/* FP:source_map.rs-0805 */                     if i == 0 {
/* FP:source_map.rs-0806 */                         return Ok(false);
/* FP:source_map.rs-0807 */                     }
/* FP:source_map.rs-0808 */                     i = i - 1;
/* FP:source_map.rs-0809 */                 } else if cc == '<' {
/* FP:source_map.rs-0810 */                     // found < in the left
/* FP:source_map.rs-0811 */                     if !found_right_angle {
/* FP:source_map.rs-0812 */                         // skip something like "(< )>"
/* FP:source_map.rs-0813 */                         return Ok(false);
/* FP:source_map.rs-0814 */                     }
/* FP:source_map.rs-0815 */                     break;
/* FP:source_map.rs-0816 */                 } else if cc == '(' {
/* FP:source_map.rs-0817 */                     if !found_right_parentheses {
/* FP:source_map.rs-0818 */                         // skip something like "<(>)"
/* FP:source_map.rs-0819 */                         return Ok(false);
/* FP:source_map.rs-0820 */                     }
/* FP:source_map.rs-0821 */                     break;
/* FP:source_map.rs-0822 */                 } else {
/* FP:source_map.rs-0823 */                     // failed to find '<' return false immediately
/* FP:source_map.rs-0824 */                     return Ok(false);
/* FP:source_map.rs-0825 */                 }
/* FP:source_map.rs-0826 */             }
/* FP:source_map.rs-0827 */             Ok(true)
/* FP:source_map.rs-0828 */         })
/* FP:source_map.rs-0829 */         .is_ok_and(|is_accessible| is_accessible)
/* FP:source_map.rs-0830 */     }
/* FP:source_map.rs-0831 */ 
/* FP:source_map.rs-0832 */     /// Given a `Span`, tries to get a shorter span ending just after the first occurrence of `char`
/* FP:source_map.rs-0833 */     /// `c`.
/* FP:source_map.rs-0834 */     pub fn span_through_char(&self, sp: Span, c: char) -> Span {
/* FP:source_map.rs-0835 */         if let Ok(snippet) = self.span_to_snippet(sp)
/* FP:source_map.rs-0836 */             && let Some(offset) = snippet.find(c)
/* FP:source_map.rs-0837 */         {
/* FP:source_map.rs-0838 */             return sp.with_hi(BytePos(sp.lo().0 + (offset + c.len_utf8()) as u32));
/* FP:source_map.rs-0839 */         }
/* FP:source_map.rs-0840 */         sp
/* FP:source_map.rs-0841 */     }
/* FP:source_map.rs-0842 */ 
/* FP:source_map.rs-0843 */     /// Given a `Span`, gets a new `Span` covering the first token and all its trailing whitespace
/* FP:source_map.rs-0844 */     /// or the original `Span`.
/* FP:source_map.rs-0845 */     ///
/* FP:source_map.rs-0846 */     /// If `sp` points to `"let mut x"`, then a span pointing at `"let "` will be returned.
/* FP:source_map.rs-0847 */     pub fn span_until_non_whitespace(&self, sp: Span) -> Span {
/* FP:source_map.rs-0848 */         let mut whitespace_found = false;
/* FP:source_map.rs-0849 */ 
/* FP:source_map.rs-0850 */         self.span_take_while(sp, |c| {
/* FP:source_map.rs-0851 */             if !whitespace_found && c.is_whitespace() {
/* FP:source_map.rs-0852 */                 whitespace_found = true;
/* FP:source_map.rs-0853 */             }
/* FP:source_map.rs-0854 */ 
/* FP:source_map.rs-0855 */             !whitespace_found || c.is_whitespace()
/* FP:source_map.rs-0856 */         })
/* FP:source_map.rs-0857 */     }
/* FP:source_map.rs-0858 */ 
/* FP:source_map.rs-0859 */     /// Given a `Span`, gets a new `Span` covering the first token without its trailing whitespace
/* FP:source_map.rs-0860 */     /// or the original `Span` in case of error.
/* FP:source_map.rs-0861 */     ///
/* FP:source_map.rs-0862 */     /// If `sp` points to `"let mut x"`, then a span pointing at `"let"` will be returned.
/* FP:source_map.rs-0863 */     pub fn span_until_whitespace(&self, sp: Span) -> Span {
/* FP:source_map.rs-0864 */         self.span_take_while(sp, |c| !c.is_whitespace())
/* FP:source_map.rs-0865 */     }
/* FP:source_map.rs-0866 */ 
/* FP:source_map.rs-0867 */     /// Given a `Span`, gets a shorter one until `predicate` yields `false`.
/* FP:source_map.rs-0868 */     pub fn span_take_while<P>(&self, sp: Span, predicate: P) -> Span
/* FP:source_map.rs-0869 */     where
/* FP:source_map.rs-0870 */         P: for<'r> FnMut(&'r char) -> bool,
/* FP:source_map.rs-0871 */     {
/* FP:source_map.rs-0872 */         if let Ok(snippet) = self.span_to_snippet(sp) {
/* FP:source_map.rs-0873 */             let offset = snippet.chars().take_while(predicate).map(|c| c.len_utf8()).sum::<usize>();
/* FP:source_map.rs-0874 */ 
/* FP:source_map.rs-0875 */             sp.with_hi(BytePos(sp.lo().0 + (offset as u32)))
/* FP:source_map.rs-0876 */         } else {
/* FP:source_map.rs-0877 */             sp
/* FP:source_map.rs-0878 */         }
/* FP:source_map.rs-0879 */     }
/* FP:source_map.rs-0880 */ 
/* FP:source_map.rs-0881 */     /// Given a `Span`, return a span ending in the closest `{`. This is useful when you have a
/* FP:source_map.rs-0882 */     /// `Span` enclosing a whole item but we need to point at only the head (usually the first
/* FP:source_map.rs-0883 */     /// line) of that item.
/* FP:source_map.rs-0884 */     ///
/* FP:source_map.rs-0885 */     /// *Only suitable for diagnostics.*
/* FP:source_map.rs-0886 */     pub fn guess_head_span(&self, sp: Span) -> Span {
/* FP:source_map.rs-0887 */         // FIXME: extend the AST items to have a head span, or replace callers with pointing at
/* FP:source_map.rs-0888 */         // the item's ident when appropriate.
/* FP:source_map.rs-0889 */         self.span_until_char(sp, '{')
/* FP:source_map.rs-0890 */     }
/* FP:source_map.rs-0891 */ 
/* FP:source_map.rs-0892 */     /// Returns a new span representing just the first character of the given span.
/* FP:source_map.rs-0893 */     pub fn start_point(&self, sp: Span) -> Span {
/* FP:source_map.rs-0894 */         let width = {
/* FP:source_map.rs-0895 */             let sp = sp.data();
/* FP:source_map.rs-0896 */             let local_begin = self.lookup_byte_offset(sp.lo);
/* FP:source_map.rs-0897 */             let start_index = local_begin.pos.to_usize();
/* FP:source_map.rs-0898 */             let src = local_begin.sf.external_src.read();
/* FP:source_map.rs-0899 */ 
/* FP:source_map.rs-0900 */             let snippet = if let Some(ref src) = local_begin.sf.src {
/* FP:source_map.rs-0901 */                 Some(&src[start_index..])
/* FP:source_map.rs-0902 */             } else {
/* FP:source_map.rs-0903 */                 src.get_source().map(|src| &src[start_index..])
/* FP:source_map.rs-0904 */             };
/* FP:source_map.rs-0905 */ 
/* FP:source_map.rs-0906 */             match snippet {
/* FP:source_map.rs-0907 */                 None => 1,
/* FP:source_map.rs-0908 */                 Some(snippet) => match snippet.chars().next() {
/* FP:source_map.rs-0909 */                     None => 1,
/* FP:source_map.rs-0910 */                     Some(c) => c.len_utf8(),
/* FP:source_map.rs-0911 */                 },
/* FP:source_map.rs-0912 */             }
/* FP:source_map.rs-0913 */         };
/* FP:source_map.rs-0914 */ 
/* FP:source_map.rs-0915 */         sp.with_hi(BytePos(sp.lo().0 + width as u32))
/* FP:source_map.rs-0916 */     }
/* FP:source_map.rs-0917 */ 
/* FP:source_map.rs-0918 */     /// Returns a new span representing just the last character of this span.
/* FP:source_map.rs-0919 */     pub fn end_point(&self, sp: Span) -> Span {
/* FP:source_map.rs-0920 */         let sp = sp.data();
/* FP:source_map.rs-0921 */         let pos = sp.hi.0;
/* FP:source_map.rs-0922 */ 
/* FP:source_map.rs-0923 */         let width = self.find_width_of_character_at_span(sp, false);
/* FP:source_map.rs-0924 */         let corrected_end_position = pos.checked_sub(width).unwrap_or(pos);
/* FP:source_map.rs-0925 */ 
/* FP:source_map.rs-0926 */         let end_point = BytePos(cmp::max(corrected_end_position, sp.lo.0));
/* FP:source_map.rs-0927 */         sp.with_lo(end_point)
/* FP:source_map.rs-0928 */     }
/* FP:source_map.rs-0929 */ 
/* FP:source_map.rs-0930 */     /// Returns a new span representing the next character after the end-point of this span.
/* FP:source_map.rs-0931 */     /// Special cases:
/* FP:source_map.rs-0932 */     /// - if span is a dummy one, returns the same span
/* FP:source_map.rs-0933 */     /// - if next_point reached the end of source, return a span exceeding the end of source,
/* FP:source_map.rs-0934 */     ///   which means sm.span_to_snippet(next_point) will get `Err`
/* FP:source_map.rs-0935 */     /// - respect multi-byte characters
/* FP:source_map.rs-0936 */     pub fn next_point(&self, sp: Span) -> Span {
/* FP:source_map.rs-0937 */         if sp.is_dummy() {
/* FP:source_map.rs-0938 */             return sp;
/* FP:source_map.rs-0939 */         }
/* FP:source_map.rs-0940 */ 
/* FP:source_map.rs-0941 */         let sp = sp.data();
/* FP:source_map.rs-0942 */         let start_of_next_point = sp.hi.0;
/* FP:source_map.rs-0943 */         let width = self.find_width_of_character_at_span(sp, true);
/* FP:source_map.rs-0944 */         // If the width is 1, then the next span should only contain the next char besides current ending.
/* FP:source_map.rs-0945 */         // However, in the case of a multibyte character, where the width != 1, the next span should
/* FP:source_map.rs-0946 */         // span multiple bytes to include the whole character.
/* FP:source_map.rs-0947 */         let end_of_next_point =
/* FP:source_map.rs-0948 */             start_of_next_point.checked_add(width).unwrap_or(start_of_next_point);
/* FP:source_map.rs-0949 */ 
/* FP:source_map.rs-0950 */         let end_of_next_point = BytePos(cmp::max(start_of_next_point + 1, end_of_next_point));
/* FP:source_map.rs-0951 */         Span::new(BytePos(start_of_next_point), end_of_next_point, sp.ctxt, None)
/* FP:source_map.rs-0952 */     }
/* FP:source_map.rs-0953 */ 
/* FP:source_map.rs-0954 */     /// Check whether span is followed by some specified expected string in limit scope
/* FP:source_map.rs-0955 */     pub fn span_look_ahead(&self, span: Span, expect: &str, limit: Option<usize>) -> Option<Span> {
/* FP:source_map.rs-0956 */         let mut sp = span;
/* FP:source_map.rs-0957 */         for _ in 0..limit.unwrap_or(100_usize) {
/* FP:source_map.rs-0958 */             sp = self.next_point(sp);
/* FP:source_map.rs-0959 */             if let Ok(ref snippet) = self.span_to_snippet(sp) {
/* FP:source_map.rs-0960 */                 if snippet == expect {
/* FP:source_map.rs-0961 */                     return Some(sp);
/* FP:source_map.rs-0962 */                 }
/* FP:source_map.rs-0963 */                 if snippet.chars().any(|c| !c.is_whitespace()) {
/* FP:source_map.rs-0964 */                     break;
/* FP:source_map.rs-0965 */                 }
/* FP:source_map.rs-0966 */             }
/* FP:source_map.rs-0967 */         }
/* FP:source_map.rs-0968 */         None
/* FP:source_map.rs-0969 */     }
/* FP:source_map.rs-0970 */ 
/* FP:source_map.rs-0971 */     /// Finds the width of the character, either before or after the end of provided span,
/* FP:source_map.rs-0972 */     /// depending on the `forwards` parameter.
/* FP:source_map.rs-0973 */     #[instrument(skip(self, sp))]
/* FP:source_map.rs-0974 */     fn find_width_of_character_at_span(&self, sp: SpanData, forwards: bool) -> u32 {
/* FP:source_map.rs-0975 */         if sp.lo == sp.hi && !forwards {
/* FP:source_map.rs-0976 */             debug!("early return empty span");
/* FP:source_map.rs-0977 */             return 1;
/* FP:source_map.rs-0978 */         }
/* FP:source_map.rs-0979 */ 
/* FP:source_map.rs-0980 */         let local_begin = self.lookup_byte_offset(sp.lo);
/* FP:source_map.rs-0981 */         let local_end = self.lookup_byte_offset(sp.hi);
/* FP:source_map.rs-0982 */         debug!("local_begin=`{:?}`, local_end=`{:?}`", local_begin, local_end);
/* FP:source_map.rs-0983 */ 
/* FP:source_map.rs-0984 */         if local_begin.sf.start_pos != local_end.sf.start_pos {
/* FP:source_map.rs-0985 */             debug!("begin and end are in different files");
/* FP:source_map.rs-0986 */             return 1;
/* FP:source_map.rs-0987 */         }
/* FP:source_map.rs-0988 */ 
/* FP:source_map.rs-0989 */         let start_index = local_begin.pos.to_usize();
/* FP:source_map.rs-0990 */         let end_index = local_end.pos.to_usize();
/* FP:source_map.rs-0991 */         debug!("start_index=`{:?}`, end_index=`{:?}`", start_index, end_index);
/* FP:source_map.rs-0992 */ 
/* FP:source_map.rs-0993 */         // Disregard indexes that are at the start or end of their spans, they can't fit bigger
/* FP:source_map.rs-0994 */         // characters.
/* FP:source_map.rs-0995 */         if (!forwards && end_index == usize::MIN) || (forwards && start_index == usize::MAX) {
/* FP:source_map.rs-0996 */             debug!("start or end of span, cannot be multibyte");
/* FP:source_map.rs-0997 */             return 1;
/* FP:source_map.rs-0998 */         }
/* FP:source_map.rs-0999 */ 
/* FP:source_map.rs-1000 */         let source_len = local_begin.sf.source_len.to_usize();
/* FP:source_map.rs-1001 */         debug!("source_len=`{:?}`", source_len);
/* FP:source_map.rs-1002 */         // Ensure indexes are also not malformed.
/* FP:source_map.rs-1003 */         if start_index > end_index || end_index > source_len - 1 {
/* FP:source_map.rs-1004 */             debug!("source indexes are malformed");
/* FP:source_map.rs-1005 */             return 1;
/* FP:source_map.rs-1006 */         }
/* FP:source_map.rs-1007 */ 
/* FP:source_map.rs-1008 */         let src = local_begin.sf.external_src.read();
/* FP:source_map.rs-1009 */ 
/* FP:source_map.rs-1010 */         let snippet = if let Some(src) = &local_begin.sf.src {
/* FP:source_map.rs-1011 */             src
/* FP:source_map.rs-1012 */         } else if let Some(src) = src.get_source() {
/* FP:source_map.rs-1013 */             src
/* FP:source_map.rs-1014 */         } else {
/* FP:source_map.rs-1015 */             return 1;
/* FP:source_map.rs-1016 */         };
/* FP:source_map.rs-1017 */ 
/* FP:source_map.rs-1018 */         if forwards {
/* FP:source_map.rs-1019 */             (snippet.ceil_char_boundary(end_index + 1) - end_index) as u32
/* FP:source_map.rs-1020 */         } else {
/* FP:source_map.rs-1021 */             (end_index - snippet.floor_char_boundary(end_index - 1)) as u32
/* FP:source_map.rs-1022 */         }
/* FP:source_map.rs-1023 */     }
/* FP:source_map.rs-1024 */ 
/* FP:source_map.rs-1025 */     pub fn get_source_file(&self, filename: &FileName) -> Option<Arc<SourceFile>> {
/* FP:source_map.rs-1026 */         // Remap filename before lookup
/* FP:source_map.rs-1027 */         let filename = self.path_mapping().map_filename_prefix(filename).0;
/* FP:source_map.rs-1028 */         for sf in self.files.borrow().source_files.iter() {
/* FP:source_map.rs-1029 */             if filename == sf.name {
/* FP:source_map.rs-1030 */                 return Some(Arc::clone(&sf));
/* FP:source_map.rs-1031 */             }
/* FP:source_map.rs-1032 */         }
/* FP:source_map.rs-1033 */         None
/* FP:source_map.rs-1034 */     }
/* FP:source_map.rs-1035 */ 
/* FP:source_map.rs-1036 */     /// For a global `BytePos`, computes the local offset within the containing `SourceFile`.
/* FP:source_map.rs-1037 */     pub fn lookup_byte_offset(&self, bpos: BytePos) -> SourceFileAndBytePos {
/* FP:source_map.rs-1038 */         let idx = self.lookup_source_file_idx(bpos);
/* FP:source_map.rs-1039 */         let sf = Arc::clone(&(*self.files.borrow().source_files)[idx]);
/* FP:source_map.rs-1040 */         let offset = bpos - sf.start_pos;
/* FP:source_map.rs-1041 */         SourceFileAndBytePos { sf, pos: offset }
/* FP:source_map.rs-1042 */     }
/* FP:source_map.rs-1043 */ 
/* FP:source_map.rs-1044 */     /// Returns the index of the [`SourceFile`] (in `self.files`) that contains `pos`.
/* FP:source_map.rs-1045 */     /// This index is guaranteed to be valid for the lifetime of this `SourceMap`,
/* FP:source_map.rs-1046 */     /// since `source_files` is a `MonotonicVec`
/* FP:source_map.rs-1047 */     pub fn lookup_source_file_idx(&self, pos: BytePos) -> usize {
/* FP:source_map.rs-1048 */         self.files.borrow().source_files.partition_point(|x| x.start_pos <= pos) - 1
/* FP:source_map.rs-1049 */     }
/* FP:source_map.rs-1050 */ 
/* FP:source_map.rs-1051 */     pub fn count_lines(&self) -> usize {
/* FP:source_map.rs-1052 */         self.files().iter().fold(0, |a, f| a + f.count_lines())
/* FP:source_map.rs-1053 */     }
/* FP:source_map.rs-1054 */ 
/* FP:source_map.rs-1055 */     pub fn ensure_source_file_source_present(&self, source_file: &SourceFile) -> bool {
/* FP:source_map.rs-1056 */         source_file.add_external_src(|| {
/* FP:source_map.rs-1057 */             let FileName::Real(ref name) = source_file.name else {
/* FP:source_map.rs-1058 */                 return None;
/* FP:source_map.rs-1059 */             };
/* FP:source_map.rs-1060 */ 
/* FP:source_map.rs-1061 */             let local_path: Cow<'_, Path> = match name {
/* FP:source_map.rs-1062 */                 RealFileName::LocalPath(local_path) => local_path.into(),
/* FP:source_map.rs-1063 */                 RealFileName::Remapped { local_path: Some(local_path), .. } => local_path.into(),
/* FP:source_map.rs-1064 */                 RealFileName::Remapped { local_path: None, virtual_name } => {
/* FP:source_map.rs-1065 */                     // The compiler produces better error messages if the sources of dependencies
/* FP:source_map.rs-1066 */                     // are available. Attempt to undo any path mapping so we can find remapped
/* FP:source_map.rs-1067 */                     // dependencies.
/* FP:source_map.rs-1068 */                     // We can only use the heuristic because `add_external_src` checks the file
/* FP:source_map.rs-1069 */                     // content hash.
/* FP:source_map.rs-1070 */                     self.path_mapping.reverse_map_prefix_heuristically(virtual_name)?.into()
/* FP:source_map.rs-1071 */                 }
/* FP:source_map.rs-1072 */             };
/* FP:source_map.rs-1073 */ 
/* FP:source_map.rs-1074 */             self.file_loader.read_file(&local_path).ok()
/* FP:source_map.rs-1075 */         })
/* FP:source_map.rs-1076 */     }
/* FP:source_map.rs-1077 */ 
/* FP:source_map.rs-1078 */     pub fn is_imported(&self, sp: Span) -> bool {
/* FP:source_map.rs-1079 */         let source_file_index = self.lookup_source_file_idx(sp.lo());
/* FP:source_map.rs-1080 */         let source_file = &self.files()[source_file_index];
/* FP:source_map.rs-1081 */         source_file.is_imported()
/* FP:source_map.rs-1082 */     }
/* FP:source_map.rs-1083 */ 
/* FP:source_map.rs-1084 */     /// Gets the span of a statement. If the statement is a macro expansion, the
/* FP:source_map.rs-1085 */     /// span in the context of the block span is found. The trailing semicolon is included
/* FP:source_map.rs-1086 */     /// on a best-effort basis.
/* FP:source_map.rs-1087 */     pub fn stmt_span(&self, stmt_span: Span, block_span: Span) -> Span {
/* FP:source_map.rs-1088 */         if !stmt_span.from_expansion() {
/* FP:source_map.rs-1089 */             return stmt_span;
/* FP:source_map.rs-1090 */         }
/* FP:source_map.rs-1091 */         let mac_call = original_sp(stmt_span, block_span);
/* FP:source_map.rs-1092 */         self.mac_call_stmt_semi_span(mac_call).map_or(mac_call, |s| mac_call.with_hi(s.hi()))
/* FP:source_map.rs-1093 */     }
/* FP:source_map.rs-1094 */ 
/* FP:source_map.rs-1095 */     /// Tries to find the span of the semicolon of a macro call statement.
/* FP:source_map.rs-1096 */     /// The input must be the *call site* span of a statement from macro expansion.
/* FP:source_map.rs-1097 */     /// ```ignore (illustrative)
/* FP:source_map.rs-1098 */     /// //       v output
/* FP:source_map.rs-1099 */     ///    mac!();
/* FP:source_map.rs-1100 */     /// // ^^^^^^ input
/* FP:source_map.rs-1101 */     /// ```
/* FP:source_map.rs-1102 */     pub fn mac_call_stmt_semi_span(&self, mac_call: Span) -> Option<Span> {
/* FP:source_map.rs-1103 */         let span = self.span_extend_while_whitespace(mac_call);
/* FP:source_map.rs-1104 */         let span = self.next_point(span);
/* FP:source_map.rs-1105 */         if self.span_to_snippet(span).as_deref() == Ok(";") { Some(span) } else { None }
/* FP:source_map.rs-1106 */     }
/* FP:source_map.rs-1107 */ }
/* FP:source_map.rs-1108 */ 
/* FP:source_map.rs-1109 */ pub fn get_source_map() -> Option<Arc<SourceMap>> {
/* FP:source_map.rs-1110 */     with_session_globals(|session_globals| session_globals.source_map.clone())
/* FP:source_map.rs-1111 */ }
/* FP:source_map.rs-1112 */ 
/* FP:source_map.rs-1113 */ #[derive(Clone)]
/* FP:source_map.rs-1114 */ pub struct FilePathMapping {
/* FP:source_map.rs-1115 */     mapping: Vec<(PathBuf, PathBuf)>,
/* FP:source_map.rs-1116 */     filename_display_for_diagnostics: FileNameDisplayPreference,
/* FP:source_map.rs-1117 */     filename_embeddable_preference: FileNameEmbeddablePreference,
/* FP:source_map.rs-1118 */ }
/* FP:source_map.rs-1119 */ 
/* FP:source_map.rs-1120 */ impl FilePathMapping {
/* FP:source_map.rs-1121 */     pub fn empty() -> FilePathMapping {
/* FP:source_map.rs-1122 */         FilePathMapping::new(
/* FP:source_map.rs-1123 */             Vec::new(),
/* FP:source_map.rs-1124 */             FileNameDisplayPreference::Local,
/* FP:source_map.rs-1125 */             FileNameEmbeddablePreference::RemappedOnly,
/* FP:source_map.rs-1126 */         )
/* FP:source_map.rs-1127 */     }
/* FP:source_map.rs-1128 */ 
/* FP:source_map.rs-1129 */     pub fn new(
/* FP:source_map.rs-1130 */         mapping: Vec<(PathBuf, PathBuf)>,
/* FP:source_map.rs-1131 */         filename_display_for_diagnostics: FileNameDisplayPreference,
/* FP:source_map.rs-1132 */         filename_embeddable_preference: FileNameEmbeddablePreference,
/* FP:source_map.rs-1133 */     ) -> FilePathMapping {
/* FP:source_map.rs-1134 */         FilePathMapping {
/* FP:source_map.rs-1135 */             mapping,
/* FP:source_map.rs-1136 */             filename_display_for_diagnostics,
/* FP:source_map.rs-1137 */             filename_embeddable_preference,
/* FP:source_map.rs-1138 */         }
/* FP:source_map.rs-1139 */     }
/* FP:source_map.rs-1140 */ 
/* FP:source_map.rs-1141 */     /// Applies any path prefix substitution as defined by the mapping.
/* FP:source_map.rs-1142 */     /// The return value is the remapped path and a boolean indicating whether
/* FP:source_map.rs-1143 */     /// the path was affected by the mapping.
/* FP:source_map.rs-1144 */     pub fn map_prefix<'a>(&'a self, path: impl Into<Cow<'a, Path>>) -> (Cow<'a, Path>, bool) {
/* FP:source_map.rs-1145 */         let path = path.into();
/* FP:source_map.rs-1146 */         if path.as_os_str().is_empty() {
/* FP:source_map.rs-1147 */             // Exit early if the path is empty and therefore there's nothing to remap.
/* FP:source_map.rs-1148 */             // This is mostly to reduce spam for `RUSTC_LOG=[remap_path_prefix]`.
/* FP:source_map.rs-1149 */             return (path, false);
/* FP:source_map.rs-1150 */         }
/* FP:source_map.rs-1151 */ 
/* FP:source_map.rs-1152 */         return remap_path_prefix(&self.mapping, path);
/* FP:source_map.rs-1153 */ 
/* FP:source_map.rs-1154 */         #[instrument(level = "debug", skip(mapping), ret)]
/* FP:source_map.rs-1155 */         fn remap_path_prefix<'a>(
/* FP:source_map.rs-1156 */             mapping: &'a [(PathBuf, PathBuf)],
/* FP:source_map.rs-1157 */             path: Cow<'a, Path>,
/* FP:source_map.rs-1158 */         ) -> (Cow<'a, Path>, bool) {
/* FP:source_map.rs-1159 */             // NOTE: We are iterating over the mapping entries from last to first
/* FP:source_map.rs-1160 */             //       because entries specified later on the command line should
/* FP:source_map.rs-1161 */             //       take precedence.
/* FP:source_map.rs-1162 */             for (from, to) in mapping.iter().rev() {
/* FP:source_map.rs-1163 */                 debug!("Trying to apply {from:?} => {to:?}");
/* FP:source_map.rs-1164 */ 
/* FP:source_map.rs-1165 */                 if let Ok(rest) = path.strip_prefix(from) {
/* FP:source_map.rs-1166 */                     let remapped = if rest.as_os_str().is_empty() {
/* FP:source_map.rs-1167 */                         // This is subtle, joining an empty path onto e.g. `foo/bar` will
/* FP:source_map.rs-1168 */                         // result in `foo/bar/`, that is, there'll be an additional directory
/* FP:source_map.rs-1169 */                         // separator at the end. This can lead to duplicated directory separators
/* FP:source_map.rs-1170 */                         // in remapped paths down the line.
/* FP:source_map.rs-1171 */                         // So, if we have an exact match, we just return that without a call
/* FP:source_map.rs-1172 */                         // to `Path::join()`.
/* FP:source_map.rs-1173 */                         to.into()
/* FP:source_map.rs-1174 */                     } else {
/* FP:source_map.rs-1175 */                         to.join(rest).into()
/* FP:source_map.rs-1176 */                     };
/* FP:source_map.rs-1177 */                     debug!("Match - remapped");
/* FP:source_map.rs-1178 */ 
/* FP:source_map.rs-1179 */                     return (remapped, true);
/* FP:source_map.rs-1180 */                 } else {
/* FP:source_map.rs-1181 */                     debug!("No match - prefix {from:?} does not match");
/* FP:source_map.rs-1182 */                 }
/* FP:source_map.rs-1183 */             }
/* FP:source_map.rs-1184 */ 
/* FP:source_map.rs-1185 */             debug!("not remapped");
/* FP:source_map.rs-1186 */             (path, false)
/* FP:source_map.rs-1187 */         }
/* FP:source_map.rs-1188 */     }
/* FP:source_map.rs-1189 */ 
/* FP:source_map.rs-1190 */     fn map_filename_prefix(&self, file: &FileName) -> (FileName, bool) {
/* FP:source_map.rs-1191 */         match file {
/* FP:source_map.rs-1192 */             FileName::Real(realfile) if let RealFileName::LocalPath(local_path) = realfile => {
/* FP:source_map.rs-1193 */                 let (mapped_path, mapped) = self.map_prefix(local_path);
/* FP:source_map.rs-1194 */                 let realfile = if mapped {
/* FP:source_map.rs-1195 */                     RealFileName::Remapped {
/* FP:source_map.rs-1196 */                         local_path: Some(local_path.clone()),
/* FP:source_map.rs-1197 */                         virtual_name: mapped_path.into_owned(),
/* FP:source_map.rs-1198 */                     }
/* FP:source_map.rs-1199 */                 } else {
/* FP:source_map.rs-1200 */                     realfile.clone()
/* FP:source_map.rs-1201 */                 };
/* FP:source_map.rs-1202 */                 (FileName::Real(realfile), mapped)
/* FP:source_map.rs-1203 */             }
/* FP:source_map.rs-1204 */             FileName::Real(_) => unreachable!("attempted to remap an already remapped filename"),
/* FP:source_map.rs-1205 */             other => (other.clone(), false),
/* FP:source_map.rs-1206 */         }
/* FP:source_map.rs-1207 */     }
/* FP:source_map.rs-1208 */ 
/* FP:source_map.rs-1209 */     /// Applies any path prefix substitution as defined by the mapping.
/* FP:source_map.rs-1210 */     /// The return value is the local path with a "virtual path" representing the remapped
/* FP:source_map.rs-1211 */     /// part if any remapping was performed.
/* FP:source_map.rs-1212 */     pub fn to_real_filename<'a>(&self, local_path: impl Into<Cow<'a, Path>>) -> RealFileName {
/* FP:source_map.rs-1213 */         let local_path = local_path.into();
/* FP:source_map.rs-1214 */         if let (remapped_path, true) = self.map_prefix(&*local_path) {
/* FP:source_map.rs-1215 */             RealFileName::Remapped {
/* FP:source_map.rs-1216 */                 virtual_name: remapped_path.into_owned(),
/* FP:source_map.rs-1217 */                 local_path: Some(local_path.into_owned()),
/* FP:source_map.rs-1218 */             }
/* FP:source_map.rs-1219 */         } else {
/* FP:source_map.rs-1220 */             RealFileName::LocalPath(local_path.into_owned())
/* FP:source_map.rs-1221 */         }
/* FP:source_map.rs-1222 */     }
/* FP:source_map.rs-1223 */ 
/* FP:source_map.rs-1224 */     /// Expand a relative path to an absolute path with remapping taken into account.
/* FP:source_map.rs-1225 */     /// Use this when absolute paths are required (e.g. debuginfo or crate metadata).
/* FP:source_map.rs-1226 */     ///
/* FP:source_map.rs-1227 */     /// The resulting `RealFileName` will have its `local_path` portion erased if
/* FP:source_map.rs-1228 */     /// possible (i.e. if there's also a remapped path).
/* FP:source_map.rs-1229 */     pub fn to_embeddable_absolute_path(
/* FP:source_map.rs-1230 */         &self,
/* FP:source_map.rs-1231 */         file_path: RealFileName,
/* FP:source_map.rs-1232 */         working_directory: &RealFileName,
/* FP:source_map.rs-1233 */     ) -> RealFileName {
/* FP:source_map.rs-1234 */         match file_path {
/* FP:source_map.rs-1235 */             // Anything that's already remapped we don't modify, except for erasing
/* FP:source_map.rs-1236 */             // the `local_path` portion (if desired).
/* FP:source_map.rs-1237 */             RealFileName::Remapped { local_path, virtual_name } => {
/* FP:source_map.rs-1238 */                 RealFileName::Remapped {
/* FP:source_map.rs-1239 */                     local_path: match self.filename_embeddable_preference {
/* FP:source_map.rs-1240 */                         FileNameEmbeddablePreference::RemappedOnly => None,
/* FP:source_map.rs-1241 */                         FileNameEmbeddablePreference::LocalAndRemapped => local_path,
/* FP:source_map.rs-1242 */                     },
/* FP:source_map.rs-1243 */                     // We use the remapped name verbatim, even if it looks like a relative
/* FP:source_map.rs-1244 */                     // path. The assumption is that the user doesn't want us to further
/* FP:source_map.rs-1245 */                     // process paths that have gone through remapping.
/* FP:source_map.rs-1246 */                     virtual_name,
/* FP:source_map.rs-1247 */                 }
/* FP:source_map.rs-1248 */             }
/* FP:source_map.rs-1249 */ 
/* FP:source_map.rs-1250 */             RealFileName::LocalPath(unmapped_file_path) => {
/* FP:source_map.rs-1251 */                 // If no remapping has been applied yet, try to do so
/* FP:source_map.rs-1252 */                 let (new_path, was_remapped) = self.map_prefix(&unmapped_file_path);
/* FP:source_map.rs-1253 */                 if was_remapped {
/* FP:source_map.rs-1254 */                     // It was remapped, so don't modify further
/* FP:source_map.rs-1255 */                     return RealFileName::Remapped {
/* FP:source_map.rs-1256 */                         virtual_name: new_path.into_owned(),
/* FP:source_map.rs-1257 */                         // But still provide the local path if desired
/* FP:source_map.rs-1258 */                         local_path: match self.filename_embeddable_preference {
/* FP:source_map.rs-1259 */                             FileNameEmbeddablePreference::RemappedOnly => None,
/* FP:source_map.rs-1260 */                             FileNameEmbeddablePreference::LocalAndRemapped => {
/* FP:source_map.rs-1261 */                                 Some(unmapped_file_path)
/* FP:source_map.rs-1262 */                             }
/* FP:source_map.rs-1263 */                         },
/* FP:source_map.rs-1264 */                     };
/* FP:source_map.rs-1265 */                 }
/* FP:source_map.rs-1266 */ 
/* FP:source_map.rs-1267 */                 if new_path.is_absolute() {
/* FP:source_map.rs-1268 */                     // No remapping has applied to this path and it is absolute,
/* FP:source_map.rs-1269 */                     // so the working directory cannot influence it either, so
/* FP:source_map.rs-1270 */                     // we are done.
/* FP:source_map.rs-1271 */                     return RealFileName::LocalPath(new_path.into_owned());
/* FP:source_map.rs-1272 */                 }
/* FP:source_map.rs-1273 */ 
/* FP:source_map.rs-1274 */                 debug_assert!(new_path.is_relative());
/* FP:source_map.rs-1275 */                 let unmapped_file_path_rel = new_path;
/* FP:source_map.rs-1276 */ 
/* FP:source_map.rs-1277 */                 match working_directory {
/* FP:source_map.rs-1278 */                     RealFileName::LocalPath(unmapped_working_dir_abs) => {
/* FP:source_map.rs-1279 */                         let unmapped_file_path_abs =
/* FP:source_map.rs-1280 */                             unmapped_working_dir_abs.join(unmapped_file_path_rel);
/* FP:source_map.rs-1281 */ 
/* FP:source_map.rs-1282 */                         // Although neither `working_directory` nor the file name were subject
/* FP:source_map.rs-1283 */                         // to path remapping, the concatenation between the two may be. Hence
/* FP:source_map.rs-1284 */                         // we need to do a remapping here.
/* FP:source_map.rs-1285 */                         let (file_path_abs, was_remapped) =
/* FP:source_map.rs-1286 */                             self.map_prefix(&unmapped_file_path_abs);
/* FP:source_map.rs-1287 */                         if was_remapped {
/* FP:source_map.rs-1288 */                             RealFileName::Remapped {
/* FP:source_map.rs-1289 */                                 virtual_name: file_path_abs.into_owned(),
/* FP:source_map.rs-1290 */                                 local_path: match self.filename_embeddable_preference {
/* FP:source_map.rs-1291 */                                     FileNameEmbeddablePreference::RemappedOnly => None,
/* FP:source_map.rs-1292 */                                     FileNameEmbeddablePreference::LocalAndRemapped => {
/* FP:source_map.rs-1293 */                                         Some(unmapped_file_path_abs)
/* FP:source_map.rs-1294 */                                     }
/* FP:source_map.rs-1295 */                                 },
/* FP:source_map.rs-1296 */                             }
/* FP:source_map.rs-1297 */                         } else {
/* FP:source_map.rs-1298 */                             // No kind of remapping applied to this path, so
/* FP:source_map.rs-1299 */                             // we leave it as it is.
/* FP:source_map.rs-1300 */                             RealFileName::LocalPath(file_path_abs.into_owned())
/* FP:source_map.rs-1301 */                         }
/* FP:source_map.rs-1302 */                     }
/* FP:source_map.rs-1303 */                     RealFileName::Remapped {
/* FP:source_map.rs-1304 */                         local_path,
/* FP:source_map.rs-1305 */                         virtual_name: remapped_working_dir_abs,
/* FP:source_map.rs-1306 */                     } => {
/* FP:source_map.rs-1307 */                         // If working_directory has been remapped, then we emit
/* FP:source_map.rs-1308 */                         // Remapped variant as the expanded path won't be valid
/* FP:source_map.rs-1309 */                         RealFileName::Remapped {
/* FP:source_map.rs-1310 */                             virtual_name: Path::new(remapped_working_dir_abs)
/* FP:source_map.rs-1311 */                                 .join(&unmapped_file_path_rel),
/* FP:source_map.rs-1312 */                             local_path: match self.filename_embeddable_preference {
/* FP:source_map.rs-1313 */                                 FileNameEmbeddablePreference::RemappedOnly => None,
/* FP:source_map.rs-1314 */                                 FileNameEmbeddablePreference::LocalAndRemapped => local_path
/* FP:source_map.rs-1315 */                                     .as_ref()
/* FP:source_map.rs-1316 */                                     .map(|local_path| local_path.join(unmapped_file_path_rel)),
/* FP:source_map.rs-1317 */                             },
/* FP:source_map.rs-1318 */                         }
/* FP:source_map.rs-1319 */                     }
/* FP:source_map.rs-1320 */                 }
/* FP:source_map.rs-1321 */             }
/* FP:source_map.rs-1322 */         }
/* FP:source_map.rs-1323 */     }
/* FP:source_map.rs-1324 */ 
/* FP:source_map.rs-1325 */     /// Attempts to (heuristically) reverse a prefix mapping.
/* FP:source_map.rs-1326 */     ///
/* FP:source_map.rs-1327 */     /// Returns [`Some`] if there is exactly one mapping where the "to" part is
/* FP:source_map.rs-1328 */     /// a prefix of `path` and has at least one non-empty
/* FP:source_map.rs-1329 */     /// [`Normal`](path::Component::Normal) component. The component
/* FP:source_map.rs-1330 */     /// restriction exists to avoid reverse mapping overly generic paths like
/* FP:source_map.rs-1331 */     /// `/` or `.`).
/* FP:source_map.rs-1332 */     ///
/* FP:source_map.rs-1333 */     /// This is a heuristic and not guaranteed to return the actual original
/* FP:source_map.rs-1334 */     /// path! Do not rely on the result unless you have other means to verify
/* FP:source_map.rs-1335 */     /// that the mapping is correct (e.g. by checking the file content hash).
/* FP:source_map.rs-1336 */     #[instrument(level = "debug", skip(self), ret)]
/* FP:source_map.rs-1337 */     fn reverse_map_prefix_heuristically(&self, path: &Path) -> Option<PathBuf> {
/* FP:source_map.rs-1338 */         let mut found = None;
/* FP:source_map.rs-1339 */ 
/* FP:source_map.rs-1340 */         for (from, to) in self.mapping.iter() {
/* FP:source_map.rs-1341 */             let has_normal_component = to.components().any(|c| match c {
/* FP:source_map.rs-1342 */                 path::Component::Normal(s) => !s.is_empty(),
/* FP:source_map.rs-1343 */                 _ => false,
/* FP:source_map.rs-1344 */             });
/* FP:source_map.rs-1345 */ 
/* FP:source_map.rs-1346 */             if !has_normal_component {
/* FP:source_map.rs-1347 */                 continue;
/* FP:source_map.rs-1348 */             }
/* FP:source_map.rs-1349 */ 
/* FP:source_map.rs-1350 */             let Ok(rest) = path.strip_prefix(to) else {
/* FP:source_map.rs-1351 */                 continue;
/* FP:source_map.rs-1352 */             };
/* FP:source_map.rs-1353 */ 
/* FP:source_map.rs-1354 */             if found.is_some() {
/* FP:source_map.rs-1355 */                 return None;
/* FP:source_map.rs-1356 */             }
/* FP:source_map.rs-1357 */ 
/* FP:source_map.rs-1358 */             found = Some(from.join(rest));
/* FP:source_map.rs-1359 */         }
/* FP:source_map.rs-1360 */ 
/* FP:source_map.rs-1361 */         found
/* FP:source_map.rs-1362 */     }
/* FP:source_map.rs-1363 */ }