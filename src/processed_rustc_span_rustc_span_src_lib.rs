/* FP:lib.rs-0001 */ // Source positions and related helper functions.
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // Important concepts in this module include:
/* FP:lib.rs-0004 */ //
/* FP:lib.rs-0005 */ // - the *span*, represented by [`SpanData`] and related types;
/* FP:lib.rs-0006 */ // - source code as represented by a [`SourceMap`]; and
/* FP:lib.rs-0007 */ // - interned strings, represented by [`Symbol`]s, with some common symbols available statically
/* FP:lib.rs-0008 */ //   in the [`sym`] module.
/* FP:lib.rs-0009 */ //
/* FP:lib.rs-0010 */ // Unlike most compilers, the span contains not only the position in the source code, but also
/* FP:lib.rs-0011 */ // various other metadata, such as the edition and macro hygiene. This metadata is stored in
/* FP:lib.rs-0012 */ // [`SyntaxContext`] and [`ExpnData`].
/* FP:lib.rs-0013 */ //
/* FP:lib.rs-0014 */ // ## Note
/* FP:lib.rs-0015 */ //
/* FP:lib.rs-0016 */ // This API is completely unstable and subject to change.
/* FP:lib.rs-0017 */ 
/* FP:lib.rs-0018 */ // tidy-alphabetical-start
/* FP:lib.rs-0019 */ #[allow(internal_features)]
/* FP:lib.rs-0020 */ #[cfg_attr(bootstrap, feature(round_char_boundary))]
/* FP:lib.rs-0021 */ #[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
/* FP:lib.rs-0022 */ #[doc(rust_logo)]
/* FP:lib.rs-0023 */ #[feature(array_windows)]
/* FP:lib.rs-0024 */ #[feature(cfg_select)]
/* FP:lib.rs-0025 */ #[feature(core_io_borrowed_buf)]
/* FP:lib.rs-0026 */ #[feature(if_let_guard)]
/* FP:lib.rs-0027 */ #[feature(map_try_insert)]
/* FP:lib.rs-0028 */ #[feature(negative_impls)]
/* FP:lib.rs-0029 */ #[feature(read_buf)]
/* FP:lib.rs-0030 */ #[feature(rustc_attrs)]
/* FP:lib.rs-0031 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0032 */ // tidy-alphabetical-end
/* FP:lib.rs-0033 */ 
/* FP:lib.rs-0034 */ // The code produced by the `Encodable`/`Decodable` derive macros refer to
/* FP:lib.rs-0035 */ // `crate::rustc_span::Span{Encoder,Decoder}`. That's fine outside this crate, but doesn't work inside
/* FP:lib.rs-0036 */ // this crate without this line making `rustc_span` available.
/* FP:lib.rs-0037 */ 
/* FP:lib.rs-0038 */ use derive_where::derive_where;
/* FP:lib.rs-0039 */ use crate::rustc_data_structures::{AtomicRef, outline};
/* FP:lib.rs-0040 */ use rustc_macros::{Decodable, Encodable, HashStable_Generic};
/* FP:lib.rs-0041 */ use crate::rustc_serialize::opaque::{FileEncoder, MemDecoder};
/* FP:lib.rs-0042 */ use crate::rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
/* FP:lib.rs-0043 */ use tracing::debug;
/* FP:lib.rs-0044 */ 
/* FP:lib.rs-0047 */ use source_map::{SourceMap, SourceMapInputs};
/* FP:lib.rs-0048 */ 
/* FP:lib.rs-0049 */ pub use self::caching_source_map_view::CachingSourceMapView;
/* FP:lib.rs-0050 */ use crate::fatal_error::FatalError;
/* FP:lib.rs-0051 */ 
/* FP:lib.rs-0053 */ use edition::Edition;
/* FP:lib.rs-0055 */ use hygiene::Transparency;
/* FP:lib.rs-0056 */ pub use hygiene::{
/* FP:lib.rs-0057 */     DesugaringKind, ExpnData, ExpnHash, ExpnId, ExpnKind, LocalExpnId, MacroKind, SyntaxContext,
/* FP:lib.rs-0058 */ };
/* FP:lib.rs-0059 */ use crate::rustc_data_structures::stable_hasher::HashingControls;
/* FP:lib.rs-0061 */ use def_id::{CrateNum, DefId, DefIndex, DefPathHash, LOCAL_CRATE, LocalDefId, StableCrateId};
/* FP:lib.rs-0064 */ pub use span_encoding::{DUMMY_SP, Span};
/* FP:lib.rs-0065 */ 
/* FP:lib.rs-0067 */ pub use symbol::{
/* FP:lib.rs-0068 */     ByteSymbol, Ident, MacroRulesNormalizedIdent, Macros20NormalizedIdent, STDLIB_STABLE_CRATES,
/* FP:lib.rs-0069 */     Symbol, kw, sym,
/* FP:lib.rs-0070 */ };
/* FP:lib.rs-0071 */ 
/* FP:lib.rs-0074 */ 
/* FP:lib.rs-0076 */ 
/* FP:lib.rs-0077 */ use std::borrow::Cow;
/* FP:lib.rs-0078 */ use std::cmp::{self, Ordering};
/* FP:lib.rs-0079 */ use std::fmt::Display;
/* FP:lib.rs-0080 */ use std::hash::Hash;
/* FP:lib.rs-0081 */ use std::io::{self, Read};
/* FP:lib.rs-0082 */ use std::ops::{Add, Range, Sub};
/* FP:lib.rs-0083 */ use std::path::{Path, PathBuf};
/* FP:lib.rs-0084 */ use std::str::FromStr;
/* FP:lib.rs-0085 */ use std::sync::Arc;
/* FP:lib.rs-0086 */ use std::{fmt, iter};
/* FP:lib.rs-0087 */ 
/* FP:lib.rs-0088 */ use md5::{Digest, Md5};
/* FP:lib.rs-0089 */ use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher};
/* FP:lib.rs-0090 */ use crate::rustc_data_structures::sync::{FreezeLock, FreezeWriteGuard, Lock};
/* FP:lib.rs-0091 */ use crate::rustc_data_structures::unord::UnordMap;
/* FP:lib.rs-0092 */ use rustc_hashes::{Hash64, Hash128};
/* FP:lib.rs-0093 */ use sha1::Sha1;
/* FP:lib.rs-0094 */ use sha2::Sha256;
/* FP:lib.rs-0095 */ 
/* FP:lib.rs-0096 */ #[cfg(test)]
/* FP:lib.rs-0098 */ 
/* FP:lib.rs-0099 */ /// Per-session global variables: this struct is stored in thread-local storage
/* FP:lib.rs-0100 */ /// in such a way that it is accessible without any kind of handle to all
/* FP:lib.rs-0101 */ /// threads within the compilation session, but is not accessible outside the
/* FP:lib.rs-0102 */ /// session.
/* FP:lib.rs-0103 */ pub struct SessionGlobals {
/* FP:lib.rs-0104 */     symbol_interner: symbol::Interner,
/* FP:lib.rs-0105 */     span_interner: Lock<span_encoding::SpanInterner>,
/* FP:lib.rs-0106 */     /// Maps a macro argument token into use of the corresponding metavariable in the macro body.
/* FP:lib.rs-0107 */     /// Collisions are possible and processed in `maybe_use_metavar_location` on best effort basis.
/* FP:lib.rs-0108 */     metavar_spans: MetavarSpansMap,
/* FP:lib.rs-0109 */     hygiene_data: Lock<hygiene::HygieneData>,
/* FP:lib.rs-0110 */ 
/* FP:lib.rs-0111 */     /// The session's source map, if there is one. This field should only be
/* FP:lib.rs-0112 */     /// used in places where the `Session` is truly not available, such as
/* FP:lib.rs-0113 */     /// `<Span as Debug>::fmt`.
/* FP:lib.rs-0114 */     source_map: Option<Arc<SourceMap>>,
/* FP:lib.rs-0115 */ }
/* FP:lib.rs-0116 */ 
/* FP:lib.rs-0117 */ impl SessionGlobals {
/* FP:lib.rs-0118 */     pub fn new(
/* FP:lib.rs-0119 */         edition: Edition,
/* FP:lib.rs-0120 */         extra_symbols: &[&'static str],
/* FP:lib.rs-0121 */         sm_inputs: Option<SourceMapInputs>,
/* FP:lib.rs-0122 */     ) -> SessionGlobals {
/* FP:lib.rs-0123 */         SessionGlobals {
/* FP:lib.rs-0124 */             symbol_interner: symbol::Interner::with_extra_symbols(extra_symbols),
/* FP:lib.rs-0125 */             span_interner: Lock::new(span_encoding::SpanInterner::default()),
/* FP:lib.rs-0126 */             metavar_spans: Default::default(),
/* FP:lib.rs-0127 */             hygiene_data: Lock::new(hygiene::HygieneData::new(edition)),
/* FP:lib.rs-0128 */             source_map: sm_inputs.map(|inputs| Arc::new(SourceMap::with_inputs(inputs))),
/* FP:lib.rs-0129 */         }
/* FP:lib.rs-0130 */     }
/* FP:lib.rs-0131 */ }
/* FP:lib.rs-0132 */ 
/* FP:lib.rs-0133 */ pub fn create_session_globals_then<R>(
/* FP:lib.rs-0134 */     edition: Edition,
/* FP:lib.rs-0135 */     extra_symbols: &[&'static str],
/* FP:lib.rs-0136 */     sm_inputs: Option<SourceMapInputs>,
/* FP:lib.rs-0137 */     f: impl FnOnce() -> R,
/* FP:lib.rs-0138 */ ) -> R {
/* FP:lib.rs-0139 */     assert!(
/* FP:lib.rs-0140 */         !SESSION_GLOBALS.is_set(),
/* FP:lib.rs-0141 */         "SESSION_GLOBALS should never be overwritten! \
/* FP:lib.rs-0142 */          Use another thread if you need another SessionGlobals"
/* FP:lib.rs-0143 */     );
/* FP:lib.rs-0144 */     let session_globals = SessionGlobals::new(edition, extra_symbols, sm_inputs);
/* FP:lib.rs-0145 */     SESSION_GLOBALS.set(&session_globals, f)
/* FP:lib.rs-0146 */ }
/* FP:lib.rs-0147 */ 
/* FP:lib.rs-0148 */ pub fn set_session_globals_then<R>(session_globals: &SessionGlobals, f: impl FnOnce() -> R) -> R {
/* FP:lib.rs-0149 */     assert!(
/* FP:lib.rs-0150 */         !SESSION_GLOBALS.is_set(),
/* FP:lib.rs-0151 */         "SESSION_GLOBALS should never be overwritten! \
/* FP:lib.rs-0152 */          Use another thread if you need another SessionGlobals"
/* FP:lib.rs-0153 */     );
/* FP:lib.rs-0154 */     SESSION_GLOBALS.set(session_globals, f)
/* FP:lib.rs-0155 */ }
/* FP:lib.rs-0156 */ 
/* FP:lib.rs-0157 */ /// No source map.
/* FP:lib.rs-0158 */ pub fn create_session_if_not_set_then<R, F>(edition: Edition, f: F) -> R
/* FP:lib.rs-0159 */ where
/* FP:lib.rs-0160 */     F: FnOnce(&SessionGlobals) -> R,
/* FP:lib.rs-0161 */ {
/* FP:lib.rs-0162 */     if !SESSION_GLOBALS.is_set() {
/* FP:lib.rs-0163 */         let session_globals = SessionGlobals::new(edition, &[], None);
/* FP:lib.rs-0164 */         SESSION_GLOBALS.set(&session_globals, || SESSION_GLOBALS.with(f))
/* FP:lib.rs-0165 */     } else {
/* FP:lib.rs-0166 */         SESSION_GLOBALS.with(f)
/* FP:lib.rs-0167 */     }
/* FP:lib.rs-0168 */ }
/* FP:lib.rs-0169 */ 
/* FP:lib.rs-0170 */ #[inline]
/* FP:lib.rs-0171 */ pub fn with_session_globals<R, F>(f: F) -> R
/* FP:lib.rs-0172 */ where
/* FP:lib.rs-0173 */     F: FnOnce(&SessionGlobals) -> R,
/* FP:lib.rs-0174 */ {
/* FP:lib.rs-0175 */     SESSION_GLOBALS.with(f)
/* FP:lib.rs-0176 */ }
/* FP:lib.rs-0177 */ 
/* FP:lib.rs-0178 */ /// Default edition, no source map.
/* FP:lib.rs-0179 */ pub fn create_default_session_globals_then<R>(f: impl FnOnce() -> R) -> R {
/* FP:lib.rs-0180 */     create_session_globals_then(edition::DEFAULT_EDITION, &[], None, f)
/* FP:lib.rs-0181 */ }
/* FP:lib.rs-0182 */ 
/* FP:lib.rs-0183 */ // If this ever becomes non thread-local, `decode_syntax_context`
/* FP:lib.rs-0184 */ // and `decode_expn_id` will need to be updated to handle concurrent
/* FP:lib.rs-0185 */ // deserialization.
/* FP:lib.rs-0186 */ scoped_tls::scoped_thread_local!(static SESSION_GLOBALS: SessionGlobals);
/* FP:lib.rs-0187 */ 
/* FP:lib.rs-0188 */ #[derive(Default)]
/* FP:lib.rs-0189 */ pub struct MetavarSpansMap(FreezeLock<UnordMap<Span, (Span, bool)>>);
/* FP:lib.rs-0190 */ 
/* FP:lib.rs-0191 */ impl MetavarSpansMap {
/* FP:lib.rs-0192 */     pub fn insert(&self, span: Span, var_span: Span) -> bool {
/* FP:lib.rs-0193 */         match self.0.write().try_insert(span, (var_span, false)) {
/* FP:lib.rs-0194 */             Ok(_) => true,
/* FP:lib.rs-0195 */             Err(entry) => entry.entry.get().0 == var_span,
/* FP:lib.rs-0196 */         }
/* FP:lib.rs-0197 */     }
/* FP:lib.rs-0198 */ 
/* FP:lib.rs-0199 */     /// Read a span and record that it was read.
/* FP:lib.rs-0200 */     pub fn get(&self, span: Span) -> Option<Span> {
/* FP:lib.rs-0201 */         if let Some(mut mspans) = self.0.try_write() {
/* FP:lib.rs-0202 */             if let Some((var_span, read)) = mspans.get_mut(&span) {
/* FP:lib.rs-0203 */                 *read = true;
/* FP:lib.rs-0204 */                 Some(*var_span)
/* FP:lib.rs-0205 */             } else {
/* FP:lib.rs-0206 */                 None
/* FP:lib.rs-0207 */             }
/* FP:lib.rs-0208 */         } else {
/* FP:lib.rs-0209 */             if let Some((span, true)) = self.0.read().get(&span) { Some(*span) } else { None }
/* FP:lib.rs-0210 */         }
/* FP:lib.rs-0211 */     }
/* FP:lib.rs-0212 */ 
/* FP:lib.rs-0213 */     /// Freeze the set, and return the spans which have been read.
/* FP:lib.rs-0214 */     ///
/* FP:lib.rs-0215 */     /// After this is frozen, no spans that have not been read can be read.
/* FP:lib.rs-0216 */     pub fn freeze_and_get_read_spans(&self) -> UnordMap<Span, Span> {
/* FP:lib.rs-0217 */         self.0.freeze().items().filter(|(_, (_, b))| *b).map(|(s1, (s2, _))| (*s1, *s2)).collect()
/* FP:lib.rs-0218 */     }
/* FP:lib.rs-0219 */ }
/* FP:lib.rs-0220 */ 
/* FP:lib.rs-0221 */ #[inline]
/* FP:lib.rs-0222 */ pub fn with_metavar_spans<R>(f: impl FnOnce(&MetavarSpansMap) -> R) -> R {
/* FP:lib.rs-0223 */     with_session_globals(|session_globals| f(&session_globals.metavar_spans))
/* FP:lib.rs-0224 */ }
/* FP:lib.rs-0225 */ 
/* FP:lib.rs-0226 */ // FIXME: We should use this enum or something like it to get rid of the
/* FP:lib.rs-0227 */ // use of magic `/rust/1.x/...` paths across the board.
/* FP:lib.rs-0228 */ #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Decodable, Encodable)]
/* FP:lib.rs-0229 */ pub enum RealFileName {
/* FP:lib.rs-0230 */     LocalPath(PathBuf),
/* FP:lib.rs-0231 */     /// For remapped paths (namely paths into libstd that have been mapped
/* FP:lib.rs-0232 */     /// to the appropriate spot on the local host's file system, and local file
/* FP:lib.rs-0233 */     /// system paths that have been remapped with `FilePathMapping`),
/* FP:lib.rs-0234 */     Remapped {
/* FP:lib.rs-0235 */         /// `local_path` is the (host-dependent) local path to the file. This is
/* FP:lib.rs-0236 */         /// None if the file was imported from another crate
/* FP:lib.rs-0237 */         local_path: Option<PathBuf>,
/* FP:lib.rs-0238 */         /// `virtual_name` is the stable path rustc will store internally within
/* FP:lib.rs-0239 */         /// build artifacts.
/* FP:lib.rs-0240 */         virtual_name: PathBuf,
/* FP:lib.rs-0241 */     },
/* FP:lib.rs-0242 */ }
/* FP:lib.rs-0243 */ 
/* FP:lib.rs-0244 */ impl Hash for RealFileName {
/* FP:lib.rs-0245 */     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
/* FP:lib.rs-0246 */         // To prevent #70924 from happening again we should only hash the
/* FP:lib.rs-0247 */         // remapped (virtualized) path if that exists. This is because
/* FP:lib.rs-0248 */         // virtualized paths to sysroot crates (/rust/$hash or /rust/$version)
/* FP:lib.rs-0249 */         // remain stable even if the corresponding local_path changes
/* FP:lib.rs-0250 */         self.remapped_path_if_available().hash(state)
/* FP:lib.rs-0251 */     }
/* FP:lib.rs-0252 */ }
/* FP:lib.rs-0253 */ 
/* FP:lib.rs-0254 */ impl RealFileName {
/* FP:lib.rs-0255 */     /// Returns the path suitable for reading from the file system on the local host,
/* FP:lib.rs-0256 */     /// if this information exists.
/* FP:lib.rs-0257 */     /// Avoid embedding this in build artifacts; see `remapped_path_if_available()` for that.
/* FP:lib.rs-0258 */     pub fn local_path(&self) -> Option<&Path> {
/* FP:lib.rs-0259 */         match self {
/* FP:lib.rs-0260 */             RealFileName::LocalPath(p) => Some(p),
/* FP:lib.rs-0261 */             RealFileName::Remapped { local_path, virtual_name: _ } => local_path.as_deref(),
/* FP:lib.rs-0262 */         }
/* FP:lib.rs-0263 */     }
/* FP:lib.rs-0264 */ 
/* FP:lib.rs-0265 */     /// Returns the path suitable for reading from the file system on the local host,
/* FP:lib.rs-0266 */     /// if this information exists.
/* FP:lib.rs-0267 */     /// Avoid embedding this in build artifacts; see `remapped_path_if_available()` for that.
/* FP:lib.rs-0268 */     pub fn into_local_path(self) -> Option<PathBuf> {
/* FP:lib.rs-0269 */         match self {
/* FP:lib.rs-0270 */             RealFileName::LocalPath(p) => Some(p),
/* FP:lib.rs-0271 */             RealFileName::Remapped { local_path: p, virtual_name: _ } => p,
/* FP:lib.rs-0272 */         }
/* FP:lib.rs-0273 */     }
/* FP:lib.rs-0274 */ 
/* FP:lib.rs-0275 */     /// Returns the path suitable for embedding into build artifacts. This would still
/* FP:lib.rs-0276 */     /// be a local path if it has not been remapped. A remapped path will not correspond
/* FP:lib.rs-0277 */     /// to a valid file system path: see `local_path_if_available()` for something that
/* FP:lib.rs-0278 */     /// is more likely to return paths into the local host file system.
/* FP:lib.rs-0279 */     pub fn remapped_path_if_available(&self) -> &Path {
/* FP:lib.rs-0280 */         match self {
/* FP:lib.rs-0281 */             RealFileName::LocalPath(p)
/* FP:lib.rs-0282 */             | RealFileName::Remapped { local_path: _, virtual_name: p } => p,
/* FP:lib.rs-0283 */         }
/* FP:lib.rs-0284 */     }
/* FP:lib.rs-0285 */ 
/* FP:lib.rs-0286 */     /// Returns the path suitable for reading from the file system on the local host,
/* FP:lib.rs-0287 */     /// if this information exists. Otherwise returns the remapped name.
/* FP:lib.rs-0288 */     /// Avoid embedding this in build artifacts; see `remapped_path_if_available()` for that.
/* FP:lib.rs-0289 */     pub fn local_path_if_available(&self) -> &Path {
/* FP:lib.rs-0290 */         match self {
/* FP:lib.rs-0291 */             RealFileName::LocalPath(path)
/* FP:lib.rs-0292 */             | RealFileName::Remapped { local_path: None, virtual_name: path }
/* FP:lib.rs-0293 */             | RealFileName::Remapped { local_path: Some(path), virtual_name: _ } => path,
/* FP:lib.rs-0294 */         }
/* FP:lib.rs-0295 */     }
/* FP:lib.rs-0296 */ 
/* FP:lib.rs-0297 */     /// Return the path remapped or not depending on the [`FileNameDisplayPreference`].
/* FP:lib.rs-0298 */     ///
/* FP:lib.rs-0299 */     /// For the purpose of this function, local and short preference are equal.
/* FP:lib.rs-0300 */     pub fn to_path(&self, display_pref: FileNameDisplayPreference) -> &Path {
/* FP:lib.rs-0301 */         match display_pref {
/* FP:lib.rs-0302 */             FileNameDisplayPreference::Local | FileNameDisplayPreference::Short => {
/* FP:lib.rs-0303 */                 self.local_path_if_available()
/* FP:lib.rs-0304 */             }
/* FP:lib.rs-0305 */             FileNameDisplayPreference::Remapped => self.remapped_path_if_available(),
/* FP:lib.rs-0306 */         }
/* FP:lib.rs-0307 */     }
/* FP:lib.rs-0308 */ 
/* FP:lib.rs-0309 */     pub fn to_string_lossy(&self, display_pref: FileNameDisplayPreference) -> Cow<'_, str> {
/* FP:lib.rs-0310 */         match display_pref {
/* FP:lib.rs-0311 */             FileNameDisplayPreference::Local => self.local_path_if_available().to_string_lossy(),
/* FP:lib.rs-0312 */             FileNameDisplayPreference::Remapped => {
/* FP:lib.rs-0313 */                 self.remapped_path_if_available().to_string_lossy()
/* FP:lib.rs-0314 */             }
/* FP:lib.rs-0315 */             FileNameDisplayPreference::Short => self
/* FP:lib.rs-0316 */                 .local_path_if_available()
/* FP:lib.rs-0317 */                 .file_name()
/* FP:lib.rs-0318 */                 .map_or_else(|| "".into(), |f| f.to_string_lossy()),
/* FP:lib.rs-0319 */         }
/* FP:lib.rs-0320 */     }
/* FP:lib.rs-0321 */ }
/* FP:lib.rs-0322 */ 
/* FP:lib.rs-0323 */ /// Differentiates between real files and common virtual files.
/* FP:lib.rs-0324 */ #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Decodable, Encodable)]
/* FP:lib.rs-0325 */ pub enum FileName {
/* FP:lib.rs-0326 */     Real(RealFileName),
/* FP:lib.rs-0327 */     /// Strings provided as `--cfg [cfgspec]`.
/* FP:lib.rs-0328 */     CfgSpec(Hash64),
/* FP:lib.rs-0329 */     /// Command line.
/* FP:lib.rs-0330 */     Anon(Hash64),
/* FP:lib.rs-0331 */     /// Hack in `src/librustc_ast/parse.rs`.
/* FP:lib.rs-0332 */     // FIXME(jseyfried)
/* FP:lib.rs-0333 */     MacroExpansion(Hash64),
/* FP:lib.rs-0334 */     ProcMacroSourceCode(Hash64),
/* FP:lib.rs-0335 */     /// Strings provided as crate attributes in the CLI.
/* FP:lib.rs-0336 */     CliCrateAttr(Hash64),
/* FP:lib.rs-0337 */     /// Custom sources for explicit parser calls from plugins and drivers.
/* FP:lib.rs-0338 */     Custom(String),
/* FP:lib.rs-0339 */     DocTest(PathBuf, isize),
/* FP:lib.rs-0340 */     /// Post-substitution inline assembly from LLVM.
/* FP:lib.rs-0341 */     InlineAsm(Hash64),
/* FP:lib.rs-0342 */ }
/* FP:lib.rs-0343 */ 
/* FP:lib.rs-0344 */ impl From<PathBuf> for FileName {
/* FP:lib.rs-0345 */     fn from(p: PathBuf) -> Self {
/* FP:lib.rs-0346 */         FileName::Real(RealFileName::LocalPath(p))
/* FP:lib.rs-0347 */     }
/* FP:lib.rs-0348 */ }
/* FP:lib.rs-0349 */ 
/* FP:lib.rs-0350 */ #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
/* FP:lib.rs-0351 */ pub enum FileNameEmbeddablePreference {
/* FP:lib.rs-0352 */     /// If a remapped path is available, only embed the `virtual_path` and omit the `local_path`.
/* FP:lib.rs-0353 */     ///
/* FP:lib.rs-0354 */     /// Otherwise embed the local-path into the `virtual_path`.
/* FP:lib.rs-0355 */     RemappedOnly,
/* FP:lib.rs-0356 */     /// Embed the original path as well as its remapped `virtual_path` component if available.
/* FP:lib.rs-0357 */     LocalAndRemapped,
/* FP:lib.rs-0358 */ }
/* FP:lib.rs-0359 */ 
/* FP:lib.rs-0360 */ #[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
/* FP:lib.rs-0361 */ pub enum FileNameDisplayPreference {
/* FP:lib.rs-0362 */     /// Display the path after the application of rewrite rules provided via `--remap-path-prefix`.
/* FP:lib.rs-0363 */     /// This is appropriate for paths that get embedded into files produced by the compiler.
/* FP:lib.rs-0364 */     Remapped,
/* FP:lib.rs-0365 */     /// Display the path before the application of rewrite rules provided via `--remap-path-prefix`.
/* FP:lib.rs-0366 */     /// This is appropriate for use in user-facing output (such as diagnostics).
/* FP:lib.rs-0367 */     Local,
/* FP:lib.rs-0368 */     /// Display only the filename, as a way to reduce the verbosity of the output.
/* FP:lib.rs-0369 */     /// This is appropriate for use in user-facing output (such as diagnostics).
/* FP:lib.rs-0370 */     Short,
/* FP:lib.rs-0371 */ }
/* FP:lib.rs-0372 */ 
/* FP:lib.rs-0373 */ pub struct FileNameDisplay<'a> {
/* FP:lib.rs-0374 */     inner: &'a FileName,
/* FP:lib.rs-0375 */     display_pref: FileNameDisplayPreference,
/* FP:lib.rs-0376 */ }
/* FP:lib.rs-0377 */ 
/* FP:lib.rs-0378 */ impl fmt::Display for FileNameDisplay<'_> {
/* FP:lib.rs-0379 */     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:lib.rs-0380 */         use FileName::*;
/* FP:lib.rs-0381 */         match *self.inner {
/* FP:lib.rs-0382 */             Real(ref name) => {
/* FP:lib.rs-0383 */                 write!(fmt, "{}", name.to_string_lossy(self.display_pref))
/* FP:lib.rs-0384 */             }
/* FP:lib.rs-0385 */             CfgSpec(_) => write!(fmt, "<cfgspec>"),
/* FP:lib.rs-0386 */             MacroExpansion(_) => write!(fmt, "<macro expansion>"),
/* FP:lib.rs-0387 */             Anon(_) => write!(fmt, "<anon>"),
/* FP:lib.rs-0388 */             ProcMacroSourceCode(_) => write!(fmt, "<proc-macro source code>"),
/* FP:lib.rs-0389 */             CliCrateAttr(_) => write!(fmt, "<crate attribute>"),
/* FP:lib.rs-0390 */             Custom(ref s) => write!(fmt, "<{s}>"),
/* FP:lib.rs-0391 */             DocTest(ref path, _) => write!(fmt, "{}", path.display()),
/* FP:lib.rs-0392 */             InlineAsm(_) => write!(fmt, "<inline asm>"),
/* FP:lib.rs-0393 */         }
/* FP:lib.rs-0394 */     }
/* FP:lib.rs-0395 */ }
/* FP:lib.rs-0396 */ 
/* FP:lib.rs-0397 */ impl<'a> FileNameDisplay<'a> {
/* FP:lib.rs-0398 */     pub fn to_string_lossy(&self) -> Cow<'a, str> {
/* FP:lib.rs-0399 */         match self.inner {
/* FP:lib.rs-0400 */             FileName::Real(inner) => inner.to_string_lossy(self.display_pref),
/* FP:lib.rs-0401 */             _ => Cow::from(self.to_string()),
/* FP:lib.rs-0402 */         }
/* FP:lib.rs-0403 */     }
/* FP:lib.rs-0404 */ }
/* FP:lib.rs-0405 */ 
/* FP:lib.rs-0406 */ impl FileName {
/* FP:lib.rs-0407 */     pub fn is_real(&self) -> bool {
/* FP:lib.rs-0408 */         use FileName::*;
/* FP:lib.rs-0409 */         match *self {
/* FP:lib.rs-0410 */             Real(_) => true,
/* FP:lib.rs-0411 */             Anon(_)
/* FP:lib.rs-0412 */             | MacroExpansion(_)
/* FP:lib.rs-0413 */             | ProcMacroSourceCode(_)
/* FP:lib.rs-0414 */             | CliCrateAttr(_)
/* FP:lib.rs-0415 */             | Custom(_)
/* FP:lib.rs-0416 */             | CfgSpec(_)
/* FP:lib.rs-0417 */             | DocTest(_, _)
/* FP:lib.rs-0418 */             | InlineAsm(_) => false,
/* FP:lib.rs-0419 */         }
/* FP:lib.rs-0420 */     }
/* FP:lib.rs-0421 */ 
/* FP:lib.rs-0422 */     pub fn prefer_remapped_unconditionally(&self) -> FileNameDisplay<'_> {
/* FP:lib.rs-0423 */         FileNameDisplay { inner: self, display_pref: FileNameDisplayPreference::Remapped }
/* FP:lib.rs-0424 */     }
/* FP:lib.rs-0425 */ 
/* FP:lib.rs-0426 */     /// This may include transient local filesystem information.
/* FP:lib.rs-0427 */     /// Must not be embedded in build outputs.
/* FP:lib.rs-0428 */     pub fn prefer_local(&self) -> FileNameDisplay<'_> {
/* FP:lib.rs-0429 */         FileNameDisplay { inner: self, display_pref: FileNameDisplayPreference::Local }
/* FP:lib.rs-0430 */     }
/* FP:lib.rs-0431 */ 
/* FP:lib.rs-0432 */     pub fn display(&self, display_pref: FileNameDisplayPreference) -> FileNameDisplay<'_> {
/* FP:lib.rs-0433 */         FileNameDisplay { inner: self, display_pref }
/* FP:lib.rs-0434 */     }
/* FP:lib.rs-0435 */ 
/* FP:lib.rs-0436 */     pub fn macro_expansion_source_code(src: &str) -> FileName {
/* FP:lib.rs-0437 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-0438 */         src.hash(&mut hasher);
/* FP:lib.rs-0439 */         FileName::MacroExpansion(hasher.finish())
/* FP:lib.rs-0440 */     }
/* FP:lib.rs-0441 */ 
/* FP:lib.rs-0442 */     pub fn anon_source_code(src: &str) -> FileName {
/* FP:lib.rs-0443 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-0444 */         src.hash(&mut hasher);
/* FP:lib.rs-0445 */         FileName::Anon(hasher.finish())
/* FP:lib.rs-0446 */     }
/* FP:lib.rs-0447 */ 
/* FP:lib.rs-0448 */     pub fn proc_macro_source_code(src: &str) -> FileName {
/* FP:lib.rs-0449 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-0450 */         src.hash(&mut hasher);
/* FP:lib.rs-0451 */         FileName::ProcMacroSourceCode(hasher.finish())
/* FP:lib.rs-0452 */     }
/* FP:lib.rs-0453 */ 
/* FP:lib.rs-0454 */     pub fn cfg_spec_source_code(src: &str) -> FileName {
/* FP:lib.rs-0455 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-0456 */         src.hash(&mut hasher);
/* FP:lib.rs-0457 */         FileName::CfgSpec(hasher.finish())
/* FP:lib.rs-0458 */     }
/* FP:lib.rs-0459 */ 
/* FP:lib.rs-0460 */     pub fn cli_crate_attr_source_code(src: &str) -> FileName {
/* FP:lib.rs-0461 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-0462 */         src.hash(&mut hasher);
/* FP:lib.rs-0463 */         FileName::CliCrateAttr(hasher.finish())
/* FP:lib.rs-0464 */     }
/* FP:lib.rs-0465 */ 
/* FP:lib.rs-0466 */     pub fn doc_test_source_code(path: PathBuf, line: isize) -> FileName {
/* FP:lib.rs-0467 */         FileName::DocTest(path, line)
/* FP:lib.rs-0468 */     }
/* FP:lib.rs-0469 */ 
/* FP:lib.rs-0470 */     pub fn inline_asm_source_code(src: &str) -> FileName {
/* FP:lib.rs-0471 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-0472 */         src.hash(&mut hasher);
/* FP:lib.rs-0473 */         FileName::InlineAsm(hasher.finish())
/* FP:lib.rs-0474 */     }
/* FP:lib.rs-0475 */ 
/* FP:lib.rs-0476 */     /// Returns the path suitable for reading from the file system on the local host,
/* FP:lib.rs-0477 */     /// if this information exists.
/* FP:lib.rs-0478 */     /// Avoid embedding this in build artifacts; see `remapped_path_if_available()` for that.
/* FP:lib.rs-0479 */     pub fn into_local_path(self) -> Option<PathBuf> {
/* FP:lib.rs-0480 */         match self {
/* FP:lib.rs-0481 */             FileName::Real(path) => path.into_local_path(),
/* FP:lib.rs-0482 */             FileName::DocTest(path, _) => Some(path),
/* FP:lib.rs-0483 */             _ => None,
/* FP:lib.rs-0484 */         }
/* FP:lib.rs-0485 */     }
/* FP:lib.rs-0486 */ }
/* FP:lib.rs-0487 */ 
/* FP:lib.rs-0488 */ /// Represents a span.
/* FP:lib.rs-0489 */ ///
/* FP:lib.rs-0490 */ /// Spans represent a region of code, used for error reporting. Positions in spans
/* FP:lib.rs-0491 */ /// are *absolute* positions from the beginning of the [`SourceMap`], not positions
/* FP:lib.rs-0492 */ /// relative to [`SourceFile`]s. Methods on the `SourceMap` can be used to relate spans back
/* FP:lib.rs-0493 */ /// to the original source.
/* FP:lib.rs-0494 */ ///
/* FP:lib.rs-0495 */ /// You must be careful if the span crosses more than one file, since you will not be
/* FP:lib.rs-0496 */ /// able to use many of the functions on spans in source_map and you cannot assume
/* FP:lib.rs-0497 */ /// that the length of the span is equal to `span.hi - span.lo`; there may be space in the
/* FP:lib.rs-0498 */ /// [`BytePos`] range between files.
/* FP:lib.rs-0499 */ ///
/* FP:lib.rs-0500 */ /// `SpanData` is public because `Span` uses a thread-local interner and can't be
/* FP:lib.rs-0501 */ /// sent to other threads, but some pieces of performance infra run in a separate thread.
/* FP:lib.rs-0502 */ /// Using `Span` is generally preferred.
/* FP:lib.rs-0503 */ #[derive(Clone, Copy, Hash, PartialEq, Eq)]
/* FP:lib.rs-0504 */ #[derive_where(PartialOrd, Ord)]
/* FP:lib.rs-0505 */ pub struct SpanData {
/* FP:lib.rs-0506 */     pub lo: BytePos,
/* FP:lib.rs-0507 */     pub hi: BytePos,
/* FP:lib.rs-0508 */     /// Information about where the macro came from, if this piece of
/* FP:lib.rs-0509 */     /// code was created by a macro expansion.
/* FP:lib.rs-0510 */     #[derive_where(skip)]
/* FP:lib.rs-0511 */     // `SyntaxContext` does not implement `Ord`.
/* FP:lib.rs-0512 */     // The other fields are enough to determine in-file order.
/* FP:lib.rs-0513 */     pub ctxt: SyntaxContext,
/* FP:lib.rs-0514 */     #[derive_where(skip)]
/* FP:lib.rs-0515 */     // `LocalDefId` does not implement `Ord`.
/* FP:lib.rs-0516 */     // The other fields are enough to determine in-file order.
/* FP:lib.rs-0517 */     pub parent: Option<LocalDefId>,
/* FP:lib.rs-0518 */ }
/* FP:lib.rs-0519 */ 
/* FP:lib.rs-0520 */ impl SpanData {
/* FP:lib.rs-0521 */     #[inline]
/* FP:lib.rs-0522 */     pub fn span(&self) -> Span {
/* FP:lib.rs-0523 */         Span::new(self.lo, self.hi, self.ctxt, self.parent)
/* FP:lib.rs-0524 */     }
/* FP:lib.rs-0525 */     #[inline]
/* FP:lib.rs-0526 */     pub fn with_lo(&self, lo: BytePos) -> Span {
/* FP:lib.rs-0527 */         Span::new(lo, self.hi, self.ctxt, self.parent)
/* FP:lib.rs-0528 */     }
/* FP:lib.rs-0529 */     #[inline]
/* FP:lib.rs-0530 */     pub fn with_hi(&self, hi: BytePos) -> Span {
/* FP:lib.rs-0531 */         Span::new(self.lo, hi, self.ctxt, self.parent)
/* FP:lib.rs-0532 */     }
/* FP:lib.rs-0533 */     /// Avoid if possible, `Span::map_ctxt` should be preferred.
/* FP:lib.rs-0534 */     #[inline]
/* FP:lib.rs-0535 */     fn with_ctxt(&self, ctxt: SyntaxContext) -> Span {
/* FP:lib.rs-0536 */         Span::new(self.lo, self.hi, ctxt, self.parent)
/* FP:lib.rs-0537 */     }
/* FP:lib.rs-0538 */     /// Avoid if possible, `Span::with_parent` should be preferred.
/* FP:lib.rs-0539 */     #[inline]
/* FP:lib.rs-0540 */     fn with_parent(&self, parent: Option<LocalDefId>) -> Span {
/* FP:lib.rs-0541 */         Span::new(self.lo, self.hi, self.ctxt, parent)
/* FP:lib.rs-0542 */     }
/* FP:lib.rs-0543 */     /// Returns `true` if this is a dummy span with any hygienic context.
/* FP:lib.rs-0544 */     #[inline]
/* FP:lib.rs-0545 */     pub fn is_dummy(self) -> bool {
/* FP:lib.rs-0546 */         self.lo.0 == 0 && self.hi.0 == 0
/* FP:lib.rs-0547 */     }
/* FP:lib.rs-0548 */     /// Returns `true` if `self` fully encloses `other`.
/* FP:lib.rs-0549 */     pub fn contains(self, other: Self) -> bool {
/* FP:lib.rs-0550 */         self.lo <= other.lo && other.hi <= self.hi
/* FP:lib.rs-0551 */     }
/* FP:lib.rs-0552 */ }
/* FP:lib.rs-0553 */ 
/* FP:lib.rs-0554 */ impl Default for SpanData {
/* FP:lib.rs-0555 */     fn default() -> Self {
/* FP:lib.rs-0556 */         Self { lo: BytePos(0), hi: BytePos(0), ctxt: SyntaxContext::root(), parent: None }
/* FP:lib.rs-0557 */     }
/* FP:lib.rs-0558 */ }
/* FP:lib.rs-0559 */ 
/* FP:lib.rs-0560 */ impl PartialOrd for Span {
/* FP:lib.rs-0561 */     fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
/* FP:lib.rs-0562 */         PartialOrd::partial_cmp(&self.data(), &rhs.data())
/* FP:lib.rs-0563 */     }
/* FP:lib.rs-0564 */ }
/* FP:lib.rs-0565 */ impl Ord for Span {
/* FP:lib.rs-0566 */     fn cmp(&self, rhs: &Self) -> Ordering {
/* FP:lib.rs-0567 */         Ord::cmp(&self.data(), &rhs.data())
/* FP:lib.rs-0568 */     }
/* FP:lib.rs-0569 */ }
/* FP:lib.rs-0570 */ 
/* FP:lib.rs-0571 */ impl Span {
/* FP:lib.rs-0572 */     #[inline]
/* FP:lib.rs-0573 */     pub fn lo(self) -> BytePos {
/* FP:lib.rs-0574 */         self.data().lo
/* FP:lib.rs-0575 */     }
/* FP:lib.rs-0576 */     #[inline]
/* FP:lib.rs-0577 */     pub fn with_lo(self, lo: BytePos) -> Span {
/* FP:lib.rs-0578 */         self.data().with_lo(lo)
/* FP:lib.rs-0579 */     }
/* FP:lib.rs-0580 */     #[inline]
/* FP:lib.rs-0581 */     pub fn hi(self) -> BytePos {
/* FP:lib.rs-0582 */         self.data().hi
/* FP:lib.rs-0583 */     }
/* FP:lib.rs-0584 */     #[inline]
/* FP:lib.rs-0585 */     pub fn with_hi(self, hi: BytePos) -> Span {
/* FP:lib.rs-0586 */         self.data().with_hi(hi)
/* FP:lib.rs-0587 */     }
/* FP:lib.rs-0588 */     #[inline]
/* FP:lib.rs-0589 */     pub fn with_ctxt(self, ctxt: SyntaxContext) -> Span {
/* FP:lib.rs-0590 */         self.map_ctxt(|_| ctxt)
/* FP:lib.rs-0591 */     }
/* FP:lib.rs-0592 */ 
/* FP:lib.rs-0593 */     #[inline]
/* FP:lib.rs-0594 */     pub fn is_visible(self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0595 */         !self.is_dummy() && sm.is_span_accessible(self)
/* FP:lib.rs-0596 */     }
/* FP:lib.rs-0597 */ 
/* FP:lib.rs-0598 */     /// Returns whether this span originates in a foreign crate's external macro.
/* FP:lib.rs-0599 */     ///
/* FP:lib.rs-0600 */     /// This is used to test whether a lint should not even begin to figure out whether it should
/* FP:lib.rs-0601 */     /// be reported on the current node.
/* FP:lib.rs-0602 */     #[inline]
/* FP:lib.rs-0603 */     pub fn in_external_macro(self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0604 */         self.ctxt().in_external_macro(sm)
/* FP:lib.rs-0605 */     }
/* FP:lib.rs-0606 */ 
/* FP:lib.rs-0607 */     /// Returns `true` if `span` originates in a derive-macro's expansion.
/* FP:lib.rs-0608 */     pub fn in_derive_expansion(self) -> bool {
/* FP:lib.rs-0609 */         matches!(self.ctxt().outer_expn_data().kind, ExpnKind::Macro(MacroKind::Derive, _))
/* FP:lib.rs-0610 */     }
/* FP:lib.rs-0611 */ 
/* FP:lib.rs-0612 */     /// Return whether `span` is generated by `async` or `await`.
/* FP:lib.rs-0613 */     pub fn is_from_async_await(self) -> bool {
/* FP:lib.rs-0614 */         matches!(
/* FP:lib.rs-0615 */             self.ctxt().outer_expn_data().kind,
/* FP:lib.rs-0616 */             ExpnKind::Desugaring(DesugaringKind::Async | DesugaringKind::Await),
/* FP:lib.rs-0617 */         )
/* FP:lib.rs-0618 */     }
/* FP:lib.rs-0619 */ 
/* FP:lib.rs-0620 */     /// Gate suggestions that would not be appropriate in a context the user didn't write.
/* FP:lib.rs-0621 */     pub fn can_be_used_for_suggestions(self) -> bool {
/* FP:lib.rs-0622 */         !self.from_expansion()
/* FP:lib.rs-0623 */         // FIXME: If this span comes from a `derive` macro but it points at code the user wrote,
/* FP:lib.rs-0624 */         // the callsite span and the span will be pointing at different places. It also means that
/* FP:lib.rs-0625 */         // we can safely provide suggestions on this span.
/* FP:lib.rs-0626 */             || (self.in_derive_expansion()
/* FP:lib.rs-0627 */                 && self.parent_callsite().map(|p| (p.lo(), p.hi())) != Some((self.lo(), self.hi())))
/* FP:lib.rs-0628 */     }
/* FP:lib.rs-0629 */ 
/* FP:lib.rs-0630 */     #[inline]
/* FP:lib.rs-0631 */     pub fn with_root_ctxt(lo: BytePos, hi: BytePos) -> Span {
/* FP:lib.rs-0632 */         Span::new(lo, hi, SyntaxContext::root(), None)
/* FP:lib.rs-0633 */     }
/* FP:lib.rs-0634 */ 
/* FP:lib.rs-0635 */     /// Returns a new span representing an empty span at the beginning of this span.
/* FP:lib.rs-0636 */     #[inline]
/* FP:lib.rs-0637 */     pub fn shrink_to_lo(self) -> Span {
/* FP:lib.rs-0638 */         let span = self.data_untracked();
/* FP:lib.rs-0639 */         span.with_hi(span.lo)
/* FP:lib.rs-0640 */     }
/* FP:lib.rs-0641 */     /// Returns a new span representing an empty span at the end of this span.
/* FP:lib.rs-0642 */     #[inline]
/* FP:lib.rs-0643 */     pub fn shrink_to_hi(self) -> Span {
/* FP:lib.rs-0644 */         let span = self.data_untracked();
/* FP:lib.rs-0645 */         span.with_lo(span.hi)
/* FP:lib.rs-0646 */     }
/* FP:lib.rs-0647 */ 
/* FP:lib.rs-0648 */     #[inline]
/* FP:lib.rs-0649 */     /// Returns `true` if `hi == lo`.
/* FP:lib.rs-0650 */     pub fn is_empty(self) -> bool {
/* FP:lib.rs-0651 */         let span = self.data_untracked();
/* FP:lib.rs-0652 */         span.hi == span.lo
/* FP:lib.rs-0653 */     }
/* FP:lib.rs-0654 */ 
/* FP:lib.rs-0655 */     /// Returns `self` if `self` is not the dummy span, and `other` otherwise.
/* FP:lib.rs-0656 */     pub fn substitute_dummy(self, other: Span) -> Span {
/* FP:lib.rs-0657 */         if self.is_dummy() { other } else { self }
/* FP:lib.rs-0658 */     }
/* FP:lib.rs-0659 */ 
/* FP:lib.rs-0660 */     /// Returns `true` if `self` fully encloses `other`.
/* FP:lib.rs-0661 */     pub fn contains(self, other: Span) -> bool {
/* FP:lib.rs-0662 */         let span = self.data();
/* FP:lib.rs-0663 */         let other = other.data();
/* FP:lib.rs-0664 */         span.contains(other)
/* FP:lib.rs-0665 */     }
/* FP:lib.rs-0666 */ 
/* FP:lib.rs-0667 */     /// Returns `true` if `self` touches `other`.
/* FP:lib.rs-0668 */     pub fn overlaps(self, other: Span) -> bool {
/* FP:lib.rs-0669 */         let span = self.data();
/* FP:lib.rs-0670 */         let other = other.data();
/* FP:lib.rs-0671 */         span.lo < other.hi && other.lo < span.hi
/* FP:lib.rs-0672 */     }
/* FP:lib.rs-0673 */ 
/* FP:lib.rs-0674 */     /// Returns `true` if `self` touches or adjoins `other`.
/* FP:lib.rs-0675 */     pub fn overlaps_or_adjacent(self, other: Span) -> bool {
/* FP:lib.rs-0676 */         let span = self.data();
/* FP:lib.rs-0677 */         let other = other.data();
/* FP:lib.rs-0678 */         span.lo <= other.hi && other.lo <= span.hi
/* FP:lib.rs-0679 */     }
/* FP:lib.rs-0680 */ 
/* FP:lib.rs-0681 */     /// Returns `true` if the spans are equal with regards to the source text.
/* FP:lib.rs-0682 */     ///
/* FP:lib.rs-0683 */     /// Use this instead of `==` when either span could be generated code,
/* FP:lib.rs-0684 */     /// and you only care that they point to the same bytes of source text.
/* FP:lib.rs-0685 */     pub fn source_equal(self, other: Span) -> bool {
/* FP:lib.rs-0686 */         let span = self.data();
/* FP:lib.rs-0687 */         let other = other.data();
/* FP:lib.rs-0688 */         span.lo == other.lo && span.hi == other.hi
/* FP:lib.rs-0689 */     }
/* FP:lib.rs-0690 */ 
/* FP:lib.rs-0691 */     /// Returns `Some(span)`, where the start is trimmed by the end of `other`.
/* FP:lib.rs-0692 */     pub fn trim_start(self, other: Span) -> Option<Span> {
/* FP:lib.rs-0693 */         let span = self.data();
/* FP:lib.rs-0694 */         let other = other.data();
/* FP:lib.rs-0695 */         if span.hi > other.hi { Some(span.with_lo(cmp::max(span.lo, other.hi))) } else { None }
/* FP:lib.rs-0696 */     }
/* FP:lib.rs-0697 */ 
/* FP:lib.rs-0698 */     /// Returns `Some(span)`, where the end is trimmed by the start of `other`.
/* FP:lib.rs-0699 */     pub fn trim_end(self, other: Span) -> Option<Span> {
/* FP:lib.rs-0700 */         let span = self.data();
/* FP:lib.rs-0701 */         let other = other.data();
/* FP:lib.rs-0702 */         if span.lo < other.lo { Some(span.with_hi(cmp::min(span.hi, other.lo))) } else { None }
/* FP:lib.rs-0703 */     }
/* FP:lib.rs-0704 */ 
/* FP:lib.rs-0705 */     /// Returns the source span -- this is either the supplied span, or the span for
/* FP:lib.rs-0706 */     /// the macro callsite that expanded to it.
/* FP:lib.rs-0707 */     pub fn source_callsite(self) -> Span {
/* FP:lib.rs-0708 */         let ctxt = self.ctxt();
/* FP:lib.rs-0709 */         if !ctxt.is_root() { ctxt.outer_expn_data().call_site.source_callsite() } else { self }
/* FP:lib.rs-0710 */     }
/* FP:lib.rs-0711 */ 
/* FP:lib.rs-0712 */     /// Returns the call-site span of the last macro expansion which produced this `Span`.
/* FP:lib.rs-0713 */     /// (see [`ExpnData::call_site`]). Returns `None` if this is not an expansion.
/* FP:lib.rs-0714 */     pub fn parent_callsite(self) -> Option<Span> {
/* FP:lib.rs-0715 */         let ctxt = self.ctxt();
/* FP:lib.rs-0716 */         (!ctxt.is_root()).then(|| ctxt.outer_expn_data().call_site)
/* FP:lib.rs-0717 */     }
/* FP:lib.rs-0718 */ 
/* FP:lib.rs-0719 */     /// Find the first ancestor span that's contained within `outer`.
/* FP:lib.rs-0720 */     ///
/* FP:lib.rs-0721 */     /// This method traverses the macro expansion ancestors until it finds the first span
/* FP:lib.rs-0722 */     /// that's contained within `outer`.
/* FP:lib.rs-0723 */     ///
/* FP:lib.rs-0724 */     /// The span returned by this method may have a different [`SyntaxContext`] than `outer`.
/* FP:lib.rs-0725 */     /// If you need to extend the span, use [`find_ancestor_inside_same_ctxt`] instead,
/* FP:lib.rs-0726 */     /// because joining spans with different syntax contexts can create unexpected results.
/* FP:lib.rs-0727 */     ///
/* FP:lib.rs-0728 */     /// This is used to find the span of the macro call when a parent expr span, i.e. `outer`, is known.
/* FP:lib.rs-0729 */     ///
/* FP:lib.rs-0730 */     /// [`find_ancestor_inside_same_ctxt`]: Self::find_ancestor_inside_same_ctxt
/* FP:lib.rs-0731 */     pub fn find_ancestor_inside(mut self, outer: Span) -> Option<Span> {
/* FP:lib.rs-0732 */         while !outer.contains(self) {
/* FP:lib.rs-0733 */             self = self.parent_callsite()?;
/* FP:lib.rs-0734 */         }
/* FP:lib.rs-0735 */         Some(self)
/* FP:lib.rs-0736 */     }
/* FP:lib.rs-0737 */ 
/* FP:lib.rs-0738 */     /// Find the first ancestor span with the same [`SyntaxContext`] as `other`.
/* FP:lib.rs-0739 */     ///
/* FP:lib.rs-0740 */     /// This method traverses the macro expansion ancestors until it finds a span
/* FP:lib.rs-0741 */     /// that has the same [`SyntaxContext`] as `other`.
/* FP:lib.rs-0742 */     ///
/* FP:lib.rs-0743 */     /// Like [`find_ancestor_inside_same_ctxt`], but specifically for when spans might not
/* FP:lib.rs-0744 */     /// overlap. Take care when using this, and prefer [`find_ancestor_inside`] or
/* FP:lib.rs-0745 */     /// [`find_ancestor_inside_same_ctxt`] when you know that the spans are nested (modulo
/* FP:lib.rs-0746 */     /// macro expansion).
/* FP:lib.rs-0747 */     ///
/* FP:lib.rs-0748 */     /// [`find_ancestor_inside`]: Self::find_ancestor_inside
/* FP:lib.rs-0749 */     /// [`find_ancestor_inside_same_ctxt`]: Self::find_ancestor_inside_same_ctxt
/* FP:lib.rs-0750 */     pub fn find_ancestor_in_same_ctxt(mut self, other: Span) -> Option<Span> {
/* FP:lib.rs-0751 */         while !self.eq_ctxt(other) {
/* FP:lib.rs-0752 */             self = self.parent_callsite()?;
/* FP:lib.rs-0753 */         }
/* FP:lib.rs-0754 */         Some(self)
/* FP:lib.rs-0755 */     }
/* FP:lib.rs-0756 */ 
/* FP:lib.rs-0757 */     /// Find the first ancestor span that's contained within `outer` and
/* FP:lib.rs-0758 */     /// has the same [`SyntaxContext`] as `outer`.
/* FP:lib.rs-0759 */     ///
/* FP:lib.rs-0760 */     /// This method traverses the macro expansion ancestors until it finds a span
/* FP:lib.rs-0761 */     /// that is both contained within `outer` and has the same [`SyntaxContext`] as `outer`.
/* FP:lib.rs-0762 */     ///
/* FP:lib.rs-0763 */     /// This method is the combination of [`find_ancestor_inside`] and
/* FP:lib.rs-0764 */     /// [`find_ancestor_in_same_ctxt`] and should be preferred when extending the returned span.
/* FP:lib.rs-0765 */     /// If you do not need to modify the span, use [`find_ancestor_inside`] instead.
/* FP:lib.rs-0766 */     ///
/* FP:lib.rs-0767 */     /// [`find_ancestor_inside`]: Self::find_ancestor_inside
/* FP:lib.rs-0768 */     /// [`find_ancestor_in_same_ctxt`]: Self::find_ancestor_in_same_ctxt
/* FP:lib.rs-0769 */     pub fn find_ancestor_inside_same_ctxt(mut self, outer: Span) -> Option<Span> {
/* FP:lib.rs-0770 */         while !outer.contains(self) || !self.eq_ctxt(outer) {
/* FP:lib.rs-0771 */             self = self.parent_callsite()?;
/* FP:lib.rs-0772 */         }
/* FP:lib.rs-0773 */         Some(self)
/* FP:lib.rs-0774 */     }
/* FP:lib.rs-0775 */ 
/* FP:lib.rs-0776 */     /// Find the first ancestor span that does not come from an external macro.
/* FP:lib.rs-0777 */     ///
/* FP:lib.rs-0778 */     /// This method traverses the macro expansion ancestors until it finds a span
/* FP:lib.rs-0779 */     /// that is either from user-written code or from a local macro (defined in the current crate).
/* FP:lib.rs-0780 */     ///
/* FP:lib.rs-0781 */     /// External macros are those defined in dependencies or the standard library.
/* FP:lib.rs-0782 */     /// This method is useful for reporting errors in user-controllable code and avoiding
/* FP:lib.rs-0783 */     /// diagnostics inside external macros.
/* FP:lib.rs-0784 */     ///
/* FP:lib.rs-0785 */     /// # See also
/* FP:lib.rs-0786 */     ///
/* FP:lib.rs-0787 */     /// - [`Self::find_ancestor_not_from_macro`]
/* FP:lib.rs-0788 */     /// - [`Self::in_external_macro`]
/* FP:lib.rs-0789 */     pub fn find_ancestor_not_from_extern_macro(mut self, sm: &SourceMap) -> Option<Span> {
/* FP:lib.rs-0790 */         while self.in_external_macro(sm) {
/* FP:lib.rs-0791 */             self = self.parent_callsite()?;
/* FP:lib.rs-0792 */         }
/* FP:lib.rs-0793 */         Some(self)
/* FP:lib.rs-0794 */     }
/* FP:lib.rs-0795 */ 
/* FP:lib.rs-0796 */     /// Find the first ancestor span that does not come from any macro expansion.
/* FP:lib.rs-0797 */     ///
/* FP:lib.rs-0798 */     /// This method traverses the macro expansion ancestors until it finds a span
/* FP:lib.rs-0799 */     /// that originates from user-written code rather than any macro-generated code.
/* FP:lib.rs-0800 */     ///
/* FP:lib.rs-0801 */     /// This method is useful for reporting errors at the exact location users wrote code
/* FP:lib.rs-0802 */     /// and providing suggestions at directly editable locations.
/* FP:lib.rs-0803 */     ///
/* FP:lib.rs-0804 */     /// # See also
/* FP:lib.rs-0805 */     ///
/* FP:lib.rs-0806 */     /// - [`Self::find_ancestor_not_from_extern_macro`]
/* FP:lib.rs-0807 */     /// - [`Span::from_expansion`]
/* FP:lib.rs-0808 */     pub fn find_ancestor_not_from_macro(mut self) -> Option<Span> {
/* FP:lib.rs-0809 */         while self.from_expansion() {
/* FP:lib.rs-0810 */             self = self.parent_callsite()?;
/* FP:lib.rs-0811 */         }
/* FP:lib.rs-0812 */         Some(self)
/* FP:lib.rs-0813 */     }
/* FP:lib.rs-0814 */ 
/* FP:lib.rs-0815 */     /// Edition of the crate from which this span came.
/* FP:lib.rs-0816 */     pub fn edition(self) -> edition::Edition {
/* FP:lib.rs-0817 */         self.ctxt().edition()
/* FP:lib.rs-0818 */     }
/* FP:lib.rs-0819 */ 
/* FP:lib.rs-0820 */     /// Is this edition 2015?
/* FP:lib.rs-0821 */     #[inline]
/* FP:lib.rs-0822 */     pub fn is_rust_2015(self) -> bool {
/* FP:lib.rs-0823 */         self.edition().is_rust_2015()
/* FP:lib.rs-0824 */     }
/* FP:lib.rs-0825 */ 
/* FP:lib.rs-0826 */     /// Are we allowed to use features from the Rust 2018 edition?
/* FP:lib.rs-0827 */     #[inline]
/* FP:lib.rs-0828 */     pub fn at_least_rust_2018(self) -> bool {
/* FP:lib.rs-0829 */         self.edition().at_least_rust_2018()
/* FP:lib.rs-0830 */     }
/* FP:lib.rs-0831 */ 
/* FP:lib.rs-0832 */     /// Are we allowed to use features from the Rust 2021 edition?
/* FP:lib.rs-0833 */     #[inline]
/* FP:lib.rs-0834 */     pub fn at_least_rust_2021(self) -> bool {
/* FP:lib.rs-0835 */         self.edition().at_least_rust_2021()
/* FP:lib.rs-0836 */     }
/* FP:lib.rs-0837 */ 
/* FP:lib.rs-0838 */     /// Are we allowed to use features from the Rust 2024 edition?
/* FP:lib.rs-0839 */     #[inline]
/* FP:lib.rs-0840 */     pub fn at_least_rust_2024(self) -> bool {
/* FP:lib.rs-0841 */         self.edition().at_least_rust_2024()
/* FP:lib.rs-0842 */     }
/* FP:lib.rs-0843 */ 
/* FP:lib.rs-0844 */     /// Returns the source callee.
/* FP:lib.rs-0845 */     ///
/* FP:lib.rs-0846 */     /// Returns `None` if the supplied span has no expansion trace,
/* FP:lib.rs-0847 */     /// else returns the `ExpnData` for the macro definition
/* FP:lib.rs-0848 */     /// corresponding to the source callsite.
/* FP:lib.rs-0849 */     pub fn source_callee(self) -> Option<ExpnData> {
/* FP:lib.rs-0850 */         let mut ctxt = self.ctxt();
/* FP:lib.rs-0851 */         let mut opt_expn_data = None;
/* FP:lib.rs-0852 */         while !ctxt.is_root() {
/* FP:lib.rs-0853 */             let expn_data = ctxt.outer_expn_data();
/* FP:lib.rs-0854 */             ctxt = expn_data.call_site.ctxt();
/* FP:lib.rs-0855 */             opt_expn_data = Some(expn_data);
/* FP:lib.rs-0856 */         }
/* FP:lib.rs-0857 */         opt_expn_data
/* FP:lib.rs-0858 */     }
/* FP:lib.rs-0859 */ 
/* FP:lib.rs-0860 */     /// Checks if a span is "internal" to a macro in which `#[unstable]`
/* FP:lib.rs-0861 */     /// items can be used (that is, a macro marked with
/* FP:lib.rs-0862 */     /// `#[allow_internal_unstable]`).
/* FP:lib.rs-0863 */     pub fn allows_unstable(self, feature: Symbol) -> bool {
/* FP:lib.rs-0864 */         self.ctxt()
/* FP:lib.rs-0865 */             .outer_expn_data()
/* FP:lib.rs-0866 */             .allow_internal_unstable
/* FP:lib.rs-0867 */             .is_some_and(|features| features.contains(&feature))
/* FP:lib.rs-0868 */     }
/* FP:lib.rs-0869 */ 
/* FP:lib.rs-0870 */     /// Checks if this span arises from a compiler desugaring of kind `kind`.
/* FP:lib.rs-0871 */     pub fn is_desugaring(self, kind: DesugaringKind) -> bool {
/* FP:lib.rs-0872 */         match self.ctxt().outer_expn_data().kind {
/* FP:lib.rs-0873 */             ExpnKind::Desugaring(k) => k == kind,
/* FP:lib.rs-0874 */             _ => false,
/* FP:lib.rs-0875 */         }
/* FP:lib.rs-0876 */     }
/* FP:lib.rs-0877 */ 
/* FP:lib.rs-0878 */     /// Returns the compiler desugaring that created this span, or `None`
/* FP:lib.rs-0879 */     /// if this span is not from a desugaring.
/* FP:lib.rs-0880 */     pub fn desugaring_kind(self) -> Option<DesugaringKind> {
/* FP:lib.rs-0881 */         match self.ctxt().outer_expn_data().kind {
/* FP:lib.rs-0882 */             ExpnKind::Desugaring(k) => Some(k),
/* FP:lib.rs-0883 */             _ => None,
/* FP:lib.rs-0884 */         }
/* FP:lib.rs-0885 */     }
/* FP:lib.rs-0886 */ 
/* FP:lib.rs-0887 */     /// Checks if a span is "internal" to a macro in which `unsafe`
/* FP:lib.rs-0888 */     /// can be used without triggering the `unsafe_code` lint.
/* FP:lib.rs-0889 */     /// (that is, a macro marked with `#[allow_internal_unsafe]`).
/* FP:lib.rs-0890 */     pub fn allows_unsafe(self) -> bool {
/* FP:lib.rs-0891 */         self.ctxt().outer_expn_data().allow_internal_unsafe
/* FP:lib.rs-0892 */     }
/* FP:lib.rs-0893 */ 
/* FP:lib.rs-0894 */     pub fn macro_backtrace(mut self) -> impl Iterator<Item = ExpnData> {
/* FP:lib.rs-0895 */         let mut prev_span = DUMMY_SP;
/* FP:lib.rs-0896 */         iter::from_fn(move || {
/* FP:lib.rs-0897 */             loop {
/* FP:lib.rs-0898 */                 let ctxt = self.ctxt();
/* FP:lib.rs-0899 */                 if ctxt.is_root() {
/* FP:lib.rs-0900 */                     return None;
/* FP:lib.rs-0901 */                 }
/* FP:lib.rs-0902 */ 
/* FP:lib.rs-0903 */                 let expn_data = ctxt.outer_expn_data();
/* FP:lib.rs-0904 */                 let is_recursive = expn_data.call_site.source_equal(prev_span);
/* FP:lib.rs-0905 */ 
/* FP:lib.rs-0906 */                 prev_span = self;
/* FP:lib.rs-0907 */                 self = expn_data.call_site;
/* FP:lib.rs-0908 */ 
/* FP:lib.rs-0909 */                 // Don't print recursive invocations.
/* FP:lib.rs-0910 */                 if !is_recursive {
/* FP:lib.rs-0911 */                     return Some(expn_data);
/* FP:lib.rs-0912 */                 }
/* FP:lib.rs-0913 */             }
/* FP:lib.rs-0914 */         })
/* FP:lib.rs-0915 */     }
/* FP:lib.rs-0916 */ 
/* FP:lib.rs-0917 */     /// Splits a span into two composite spans around a certain position.
/* FP:lib.rs-0918 */     pub fn split_at(self, pos: u32) -> (Span, Span) {
/* FP:lib.rs-0919 */         let len = self.hi().0 - self.lo().0;
/* FP:lib.rs-0920 */         debug_assert!(pos <= len);
/* FP:lib.rs-0921 */ 
/* FP:lib.rs-0922 */         let split_pos = BytePos(self.lo().0 + pos);
/* FP:lib.rs-0923 */         (
/* FP:lib.rs-0924 */             Span::new(self.lo(), split_pos, self.ctxt(), self.parent()),
/* FP:lib.rs-0925 */             Span::new(split_pos, self.hi(), self.ctxt(), self.parent()),
/* FP:lib.rs-0926 */         )
/* FP:lib.rs-0927 */     }
/* FP:lib.rs-0928 */ 
/* FP:lib.rs-0929 */     /// Check if you can select metavar spans for the given spans to get matching contexts.
/* FP:lib.rs-0930 */     fn try_metavars(a: SpanData, b: SpanData, a_orig: Span, b_orig: Span) -> (SpanData, SpanData) {
/* FP:lib.rs-0931 */         match with_metavar_spans(|mspans| (mspans.get(a_orig), mspans.get(b_orig))) {
/* FP:lib.rs-0932 */             (None, None) => {}
/* FP:lib.rs-0933 */             (Some(meta_a), None) => {
/* FP:lib.rs-0934 */                 let meta_a = meta_a.data();
/* FP:lib.rs-0935 */                 if meta_a.ctxt == b.ctxt {
/* FP:lib.rs-0936 */                     return (meta_a, b);
/* FP:lib.rs-0937 */                 }
/* FP:lib.rs-0938 */             }
/* FP:lib.rs-0939 */             (None, Some(meta_b)) => {
/* FP:lib.rs-0940 */                 let meta_b = meta_b.data();
/* FP:lib.rs-0941 */                 if a.ctxt == meta_b.ctxt {
/* FP:lib.rs-0942 */                     return (a, meta_b);
/* FP:lib.rs-0943 */                 }
/* FP:lib.rs-0944 */             }
/* FP:lib.rs-0945 */             (Some(meta_a), Some(meta_b)) => {
/* FP:lib.rs-0946 */                 let meta_b = meta_b.data();
/* FP:lib.rs-0947 */                 if a.ctxt == meta_b.ctxt {
/* FP:lib.rs-0948 */                     return (a, meta_b);
/* FP:lib.rs-0949 */                 }
/* FP:lib.rs-0950 */                 let meta_a = meta_a.data();
/* FP:lib.rs-0951 */                 if meta_a.ctxt == b.ctxt {
/* FP:lib.rs-0952 */                     return (meta_a, b);
/* FP:lib.rs-0953 */                 } else if meta_a.ctxt == meta_b.ctxt {
/* FP:lib.rs-0954 */                     return (meta_a, meta_b);
/* FP:lib.rs-0955 */                 }
/* FP:lib.rs-0956 */             }
/* FP:lib.rs-0957 */         }
/* FP:lib.rs-0958 */ 
/* FP:lib.rs-0959 */         (a, b)
/* FP:lib.rs-0960 */     }
/* FP:lib.rs-0961 */ 
/* FP:lib.rs-0962 */     /// Prepare two spans to a combine operation like `to` or `between`.
/* FP:lib.rs-0963 */     fn prepare_to_combine(
/* FP:lib.rs-0964 */         a_orig: Span,
/* FP:lib.rs-0965 */         b_orig: Span,
/* FP:lib.rs-0966 */     ) -> Result<(SpanData, SpanData, Option<LocalDefId>), Span> {
/* FP:lib.rs-0967 */         let (a, b) = (a_orig.data(), b_orig.data());
/* FP:lib.rs-0968 */         if a.ctxt == b.ctxt {
/* FP:lib.rs-0969 */             return Ok((a, b, if a.parent == b.parent { a.parent } else { None }));
/* FP:lib.rs-0970 */         }
/* FP:lib.rs-0971 */ 
/* FP:lib.rs-0972 */         let (a, b) = Span::try_metavars(a, b, a_orig, b_orig);
/* FP:lib.rs-0973 */         if a.ctxt == b.ctxt {
/* FP:lib.rs-0974 */             return Ok((a, b, if a.parent == b.parent { a.parent } else { None }));
/* FP:lib.rs-0975 */         }
/* FP:lib.rs-0976 */ 
/* FP:lib.rs-0977 */         // Context mismatches usually happen when procedural macros combine spans copied from
/* FP:lib.rs-0978 */         // the macro input with spans produced by the macro (`Span::*_site`).
/* FP:lib.rs-0979 */         // In that case we consider the combined span to be produced by the macro and return
/* FP:lib.rs-0980 */         // the original macro-produced span as the result.
/* FP:lib.rs-0981 */         // Otherwise we just fall back to returning the first span.
/* FP:lib.rs-0982 */         // Combining locations typically doesn't make sense in case of context mismatches.
/* FP:lib.rs-0983 */         // `is_root` here is a fast path optimization.
/* FP:lib.rs-0984 */         let a_is_callsite = a.ctxt.is_root() || a.ctxt == b.span().source_callsite().ctxt();
/* FP:lib.rs-0985 */         Err(if a_is_callsite { b_orig } else { a_orig })
/* FP:lib.rs-0986 */     }
/* FP:lib.rs-0987 */ 
/* FP:lib.rs-0988 */     /// This span, but in a larger context, may switch to the metavariable span if suitable.
/* FP:lib.rs-0989 */     pub fn with_neighbor(self, neighbor: Span) -> Span {
/* FP:lib.rs-0990 */         match Span::prepare_to_combine(self, neighbor) {
/* FP:lib.rs-0991 */             Ok((this, ..)) => this.span(),
/* FP:lib.rs-0992 */             Err(_) => self,
/* FP:lib.rs-0993 */         }
/* FP:lib.rs-0994 */     }
/* FP:lib.rs-0995 */ 
/* FP:lib.rs-0996 */     /// Returns a `Span` that would enclose both `self` and `end`.
/* FP:lib.rs-0997 */     ///
/* FP:lib.rs-0998 */     /// Note that this can also be used to extend the span "backwards":
/* FP:lib.rs-0999 */     /// `start.to(end)` and `end.to(start)` return the same `Span`.
/* FP:lib.rs-1000 */     ///
/* FP:lib.rs-1001 */     /// ```text
/* FP:lib.rs-1002 */     ///     ____             ___
/* FP:lib.rs-1003 */     ///     self lorem ipsum end
/* FP:lib.rs-1004 */     ///     ^^^^^^^^^^^^^^^^^^^^
/* FP:lib.rs-1005 */     /// ```
/* FP:lib.rs-1006 */     pub fn to(self, end: Span) -> Span {
/* FP:lib.rs-1007 */         match Span::prepare_to_combine(self, end) {
/* FP:lib.rs-1008 */             Ok((from, to, parent)) => {
/* FP:lib.rs-1009 */                 Span::new(cmp::min(from.lo, to.lo), cmp::max(from.hi, to.hi), from.ctxt, parent)
/* FP:lib.rs-1010 */             }
/* FP:lib.rs-1011 */             Err(fallback) => fallback,
/* FP:lib.rs-1012 */         }
/* FP:lib.rs-1013 */     }
/* FP:lib.rs-1014 */ 
/* FP:lib.rs-1015 */     /// Returns a `Span` between the end of `self` to the beginning of `end`.
/* FP:lib.rs-1016 */     ///
/* FP:lib.rs-1017 */     /// ```text
/* FP:lib.rs-1018 */     ///     ____             ___
/* FP:lib.rs-1019 */     ///     self lorem ipsum end
/* FP:lib.rs-1020 */     ///         ^^^^^^^^^^^^^
/* FP:lib.rs-1021 */     /// ```
/* FP:lib.rs-1022 */     pub fn between(self, end: Span) -> Span {
/* FP:lib.rs-1023 */         match Span::prepare_to_combine(self, end) {
/* FP:lib.rs-1024 */             Ok((from, to, parent)) => {
/* FP:lib.rs-1025 */                 Span::new(cmp::min(from.hi, to.hi), cmp::max(from.lo, to.lo), from.ctxt, parent)
/* FP:lib.rs-1026 */             }
/* FP:lib.rs-1027 */             Err(fallback) => fallback,
/* FP:lib.rs-1028 */         }
/* FP:lib.rs-1029 */     }
/* FP:lib.rs-1030 */ 
/* FP:lib.rs-1031 */     /// Returns a `Span` from the beginning of `self` until the beginning of `end`.
/* FP:lib.rs-1032 */     ///
/* FP:lib.rs-1033 */     /// ```text
/* FP:lib.rs-1034 */     ///     ____             ___
/* FP:lib.rs-1035 */     ///     self lorem ipsum end
/* FP:lib.rs-1036 */     ///     ^^^^^^^^^^^^^^^^^
/* FP:lib.rs-1037 */     /// ```
/* FP:lib.rs-1038 */     pub fn until(self, end: Span) -> Span {
/* FP:lib.rs-1039 */         match Span::prepare_to_combine(self, end) {
/* FP:lib.rs-1040 */             Ok((from, to, parent)) => {
/* FP:lib.rs-1041 */                 Span::new(cmp::min(from.lo, to.lo), cmp::max(from.lo, to.lo), from.ctxt, parent)
/* FP:lib.rs-1042 */             }
/* FP:lib.rs-1043 */             Err(fallback) => fallback,
/* FP:lib.rs-1044 */         }
/* FP:lib.rs-1045 */     }
/* FP:lib.rs-1046 */ 
/* FP:lib.rs-1047 */     /// Returns the `Span` within the syntax context of "within". This is useful when
/* FP:lib.rs-1048 */     /// "self" is an expansion from a macro variable, since this can be used for
/* FP:lib.rs-1049 */     /// providing extra macro expansion context for certain errors.
/* FP:lib.rs-1050 */     ///
/* FP:lib.rs-1051 */     /// ```text
/* FP:lib.rs-1052 */     /// macro_rules! m {
/* FP:lib.rs-1053 */     ///     ($ident:ident) => { ($ident,) }
/* FP:lib.rs-1054 */     /// }
/* FP:lib.rs-1055 */     ///
/* FP:lib.rs-1056 */     /// m!(outer_ident);
/* FP:lib.rs-1057 */     /// ```
/* FP:lib.rs-1058 */     ///
/* FP:lib.rs-1059 */     /// If "self" is the span of the outer_ident, and "within" is the span of the `($ident,)`
/* FP:lib.rs-1060 */     /// expr, then this will return the span of the `$ident` macro variable.
/* FP:lib.rs-1061 */     pub fn within_macro(self, within: Span, sm: &SourceMap) -> Option<Span> {
/* FP:lib.rs-1062 */         match Span::prepare_to_combine(self, within) {
/* FP:lib.rs-1063 */             // Only return something if it doesn't overlap with the original span,
/* FP:lib.rs-1064 */             // and the span isn't "imported" (i.e. from unavailable sources).
/* FP:lib.rs-1065 */             // FIXME: This does limit the usefulness of the error when the macro is
/* FP:lib.rs-1066 */             // from a foreign crate; we could also take into account `-Zmacro-backtrace`,
/* FP:lib.rs-1067 */             // which doesn't redact this span (but that would mean passing in even more
/* FP:lib.rs-1068 */             // args to this function, lol).
/* FP:lib.rs-1069 */             Ok((self_, _, parent))
/* FP:lib.rs-1070 */                 if self_.hi < self.lo() || self.hi() < self_.lo && !sm.is_imported(within) =>
/* FP:lib.rs-1071 */             {
/* FP:lib.rs-1072 */                 Some(Span::new(self_.lo, self_.hi, self_.ctxt, parent))
/* FP:lib.rs-1073 */             }
/* FP:lib.rs-1074 */             _ => None,
/* FP:lib.rs-1075 */         }
/* FP:lib.rs-1076 */     }
/* FP:lib.rs-1077 */ 
/* FP:lib.rs-1078 */     pub fn from_inner(self, inner: InnerSpan) -> Span {
/* FP:lib.rs-1079 */         let span = self.data();
/* FP:lib.rs-1080 */         Span::new(
/* FP:lib.rs-1081 */             span.lo + BytePos::from_usize(inner.start),
/* FP:lib.rs-1082 */             span.lo + BytePos::from_usize(inner.end),
/* FP:lib.rs-1083 */             span.ctxt,
/* FP:lib.rs-1084 */             span.parent,
/* FP:lib.rs-1085 */         )
/* FP:lib.rs-1086 */     }
/* FP:lib.rs-1087 */ 
/* FP:lib.rs-1088 */     /// Equivalent of `Span::def_site` from the proc macro API,
/* FP:lib.rs-1089 */     /// except that the location is taken from the `self` span.
/* FP:lib.rs-1090 */     pub fn with_def_site_ctxt(self, expn_id: ExpnId) -> Span {
/* FP:lib.rs-1091 */         self.with_ctxt_from_mark(expn_id, Transparency::Opaque)
/* FP:lib.rs-1092 */     }
/* FP:lib.rs-1093 */ 
/* FP:lib.rs-1094 */     /// Equivalent of `Span::call_site` from the proc macro API,
/* FP:lib.rs-1095 */     /// except that the location is taken from the `self` span.
/* FP:lib.rs-1096 */     pub fn with_call_site_ctxt(self, expn_id: ExpnId) -> Span {
/* FP:lib.rs-1097 */         self.with_ctxt_from_mark(expn_id, Transparency::Transparent)
/* FP:lib.rs-1098 */     }
/* FP:lib.rs-1099 */ 
/* FP:lib.rs-1100 */     /// Equivalent of `Span::mixed_site` from the proc macro API,
/* FP:lib.rs-1101 */     /// except that the location is taken from the `self` span.
/* FP:lib.rs-1102 */     pub fn with_mixed_site_ctxt(self, expn_id: ExpnId) -> Span {
/* FP:lib.rs-1103 */         self.with_ctxt_from_mark(expn_id, Transparency::SemiOpaque)
/* FP:lib.rs-1104 */     }
/* FP:lib.rs-1105 */ 
/* FP:lib.rs-1106 */     /// Produces a span with the same location as `self` and context produced by a macro with the
/* FP:lib.rs-1107 */     /// given ID and transparency, assuming that macro was defined directly and not produced by
/* FP:lib.rs-1108 */     /// some other macro (which is the case for built-in and procedural macros).
/* FP:lib.rs-1109 */     fn with_ctxt_from_mark(self, expn_id: ExpnId, transparency: Transparency) -> Span {
/* FP:lib.rs-1110 */         self.with_ctxt(SyntaxContext::root().apply_mark(expn_id, transparency))
/* FP:lib.rs-1111 */     }
/* FP:lib.rs-1112 */ 
/* FP:lib.rs-1113 */     #[inline]
/* FP:lib.rs-1114 */     pub fn apply_mark(self, expn_id: ExpnId, transparency: Transparency) -> Span {
/* FP:lib.rs-1115 */         self.map_ctxt(|ctxt| ctxt.apply_mark(expn_id, transparency))
/* FP:lib.rs-1116 */     }
/* FP:lib.rs-1117 */ 
/* FP:lib.rs-1118 */     #[inline]
/* FP:lib.rs-1119 */     pub fn remove_mark(&mut self) -> ExpnId {
/* FP:lib.rs-1120 */         let mut mark = ExpnId::root();
/* FP:lib.rs-1121 */         *self = self.map_ctxt(|mut ctxt| {
/* FP:lib.rs-1122 */             mark = ctxt.remove_mark();
/* FP:lib.rs-1123 */             ctxt
/* FP:lib.rs-1124 */         });
/* FP:lib.rs-1125 */         mark
/* FP:lib.rs-1126 */     }
/* FP:lib.rs-1127 */ 
/* FP:lib.rs-1128 */     #[inline]
/* FP:lib.rs-1129 */     pub fn adjust(&mut self, expn_id: ExpnId) -> Option<ExpnId> {
/* FP:lib.rs-1130 */         let mut mark = None;
/* FP:lib.rs-1131 */         *self = self.map_ctxt(|mut ctxt| {
/* FP:lib.rs-1132 */             mark = ctxt.adjust(expn_id);
/* FP:lib.rs-1133 */             ctxt
/* FP:lib.rs-1134 */         });
/* FP:lib.rs-1135 */         mark
/* FP:lib.rs-1136 */     }
/* FP:lib.rs-1137 */ 
/* FP:lib.rs-1138 */     #[inline]
/* FP:lib.rs-1139 */     pub fn normalize_to_macros_2_0_and_adjust(&mut self, expn_id: ExpnId) -> Option<ExpnId> {
/* FP:lib.rs-1140 */         let mut mark = None;
/* FP:lib.rs-1141 */         *self = self.map_ctxt(|mut ctxt| {
/* FP:lib.rs-1142 */             mark = ctxt.normalize_to_macros_2_0_and_adjust(expn_id);
/* FP:lib.rs-1143 */             ctxt
/* FP:lib.rs-1144 */         });
/* FP:lib.rs-1145 */         mark
/* FP:lib.rs-1146 */     }
/* FP:lib.rs-1147 */ 
/* FP:lib.rs-1148 */     #[inline]
/* FP:lib.rs-1149 */     pub fn glob_adjust(&mut self, expn_id: ExpnId, glob_span: Span) -> Option<Option<ExpnId>> {
/* FP:lib.rs-1150 */         let mut mark = None;
/* FP:lib.rs-1151 */         *self = self.map_ctxt(|mut ctxt| {
/* FP:lib.rs-1152 */             mark = ctxt.glob_adjust(expn_id, glob_span);
/* FP:lib.rs-1153 */             ctxt
/* FP:lib.rs-1154 */         });
/* FP:lib.rs-1155 */         mark
/* FP:lib.rs-1156 */     }
/* FP:lib.rs-1157 */ 
/* FP:lib.rs-1158 */     #[inline]
/* FP:lib.rs-1159 */     pub fn reverse_glob_adjust(
/* FP:lib.rs-1160 */         &mut self,
/* FP:lib.rs-1161 */         expn_id: ExpnId,
/* FP:lib.rs-1162 */         glob_span: Span,
/* FP:lib.rs-1163 */     ) -> Option<Option<ExpnId>> {
/* FP:lib.rs-1164 */         let mut mark = None;
/* FP:lib.rs-1165 */         *self = self.map_ctxt(|mut ctxt| {
/* FP:lib.rs-1166 */             mark = ctxt.reverse_glob_adjust(expn_id, glob_span);
/* FP:lib.rs-1167 */             ctxt
/* FP:lib.rs-1168 */         });
/* FP:lib.rs-1169 */         mark
/* FP:lib.rs-1170 */     }
/* FP:lib.rs-1171 */ 
/* FP:lib.rs-1172 */     #[inline]
/* FP:lib.rs-1173 */     pub fn normalize_to_macros_2_0(self) -> Span {
/* FP:lib.rs-1174 */         self.map_ctxt(|ctxt| ctxt.normalize_to_macros_2_0())
/* FP:lib.rs-1175 */     }
/* FP:lib.rs-1176 */ 
/* FP:lib.rs-1177 */     #[inline]
/* FP:lib.rs-1178 */     pub fn normalize_to_macro_rules(self) -> Span {
/* FP:lib.rs-1179 */         self.map_ctxt(|ctxt| ctxt.normalize_to_macro_rules())
/* FP:lib.rs-1180 */     }
/* FP:lib.rs-1181 */ }
/* FP:lib.rs-1182 */ 
/* FP:lib.rs-1183 */ impl Default for Span {
/* FP:lib.rs-1184 */     fn default() -> Self {
/* FP:lib.rs-1185 */         DUMMY_SP
/* FP:lib.rs-1186 */     }
/* FP:lib.rs-1187 */ }
/* FP:lib.rs-1188 */ 
/* FP:lib.rs-1189 */ crate::rustc_index::newtype_index! {
/* FP:lib.rs-1190 */     #[orderable]
/* FP:lib.rs-1191 */     #[debug_format = "AttrId({})"]
/* FP:lib.rs-1192 */     pub struct AttrId {}
/* FP:lib.rs-1193 */ }
/* FP:lib.rs-1194 */ 
/* FP:lib.rs-1195 */ /// This trait is used to allow encoder specific encodings of certain types.
/* FP:lib.rs-1196 */ /// It is similar to rustc_type_ir's TyEncoder.
/* FP:lib.rs-1197 */ pub trait SpanEncoder: Encoder {
/* FP:lib.rs-1198 */     fn encode_span(&mut self, span: Span);
/* FP:lib.rs-1199 */     fn encode_symbol(&mut self, sym: Symbol);
/* FP:lib.rs-1200 */     fn encode_byte_symbol(&mut self, byte_sym: ByteSymbol);
/* FP:lib.rs-1201 */     fn encode_expn_id(&mut self, expn_id: ExpnId);
/* FP:lib.rs-1202 */     fn encode_syntax_context(&mut self, syntax_context: SyntaxContext);
/* FP:lib.rs-1203 */     /// As a local identifier, a `CrateNum` is only meaningful within its context, e.g. within a
/* FP:lib.rs-1204 */     /// tcx. Therefore, make sure to include the context when encode a `CrateNum`.
/* FP:lib.rs-1205 */     fn encode_crate_num(&mut self, crate_num: CrateNum);
/* FP:lib.rs-1206 */     fn encode_def_index(&mut self, def_index: DefIndex);
/* FP:lib.rs-1207 */     fn encode_def_id(&mut self, def_id: DefId);
/* FP:lib.rs-1208 */ }
/* FP:lib.rs-1209 */ 
/* FP:lib.rs-1210 */ impl SpanEncoder for FileEncoder {
/* FP:lib.rs-1211 */     fn encode_span(&mut self, span: Span) {
/* FP:lib.rs-1212 */         let span = span.data();
/* FP:lib.rs-1213 */         span.lo.encode(self);
/* FP:lib.rs-1214 */         span.hi.encode(self);
/* FP:lib.rs-1215 */     }
/* FP:lib.rs-1216 */ 
/* FP:lib.rs-1217 */     fn encode_symbol(&mut self, sym: Symbol) {
/* FP:lib.rs-1218 */         self.emit_str(sym.as_str());
/* FP:lib.rs-1219 */     }
/* FP:lib.rs-1220 */ 
/* FP:lib.rs-1221 */     fn encode_byte_symbol(&mut self, byte_sym: ByteSymbol) {
/* FP:lib.rs-1222 */         self.emit_byte_str(byte_sym.as_byte_str());
/* FP:lib.rs-1223 */     }
/* FP:lib.rs-1224 */ 
/* FP:lib.rs-1225 */     fn encode_expn_id(&mut self, _expn_id: ExpnId) {
/* FP:lib.rs-1226 */         panic!("cannot encode `ExpnId` with `FileEncoder`");
/* FP:lib.rs-1227 */     }
/* FP:lib.rs-1228 */ 
/* FP:lib.rs-1229 */     fn encode_syntax_context(&mut self, _syntax_context: SyntaxContext) {
/* FP:lib.rs-1230 */         panic!("cannot encode `SyntaxContext` with `FileEncoder`");
/* FP:lib.rs-1231 */     }
/* FP:lib.rs-1232 */ 
/* FP:lib.rs-1233 */     fn encode_crate_num(&mut self, crate_num: CrateNum) {
/* FP:lib.rs-1234 */         self.emit_u32(crate_num.as_u32());
/* FP:lib.rs-1235 */     }
/* FP:lib.rs-1236 */ 
/* FP:lib.rs-1237 */     fn encode_def_index(&mut self, _def_index: DefIndex) {
/* FP:lib.rs-1238 */         panic!("cannot encode `DefIndex` with `FileEncoder`");
/* FP:lib.rs-1239 */     }
/* FP:lib.rs-1240 */ 
/* FP:lib.rs-1241 */     fn encode_def_id(&mut self, def_id: DefId) {
/* FP:lib.rs-1242 */         def_id.krate.encode(self);
/* FP:lib.rs-1243 */         def_id.index.encode(self);
/* FP:lib.rs-1244 */     }
/* FP:lib.rs-1245 */ }
/* FP:lib.rs-1246 */ 
/* FP:lib.rs-1247 */ impl<E: SpanEncoder> Encodable<E> for Span {
/* FP:lib.rs-1248 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1249 */         s.encode_span(*self);
/* FP:lib.rs-1250 */     }
/* FP:lib.rs-1251 */ }
/* FP:lib.rs-1252 */ 
/* FP:lib.rs-1253 */ impl<E: SpanEncoder> Encodable<E> for Symbol {
/* FP:lib.rs-1254 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1255 */         s.encode_symbol(*self);
/* FP:lib.rs-1256 */     }
/* FP:lib.rs-1257 */ }
/* FP:lib.rs-1258 */ 
/* FP:lib.rs-1259 */ impl<E: SpanEncoder> Encodable<E> for ByteSymbol {
/* FP:lib.rs-1260 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1261 */         s.encode_byte_symbol(*self);
/* FP:lib.rs-1262 */     }
/* FP:lib.rs-1263 */ }
/* FP:lib.rs-1264 */ 
/* FP:lib.rs-1265 */ impl<E: SpanEncoder> Encodable<E> for ExpnId {
/* FP:lib.rs-1266 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1267 */         s.encode_expn_id(*self)
/* FP:lib.rs-1268 */     }
/* FP:lib.rs-1269 */ }
/* FP:lib.rs-1270 */ 
/* FP:lib.rs-1271 */ impl<E: SpanEncoder> Encodable<E> for SyntaxContext {
/* FP:lib.rs-1272 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1273 */         s.encode_syntax_context(*self)
/* FP:lib.rs-1274 */     }
/* FP:lib.rs-1275 */ }
/* FP:lib.rs-1276 */ 
/* FP:lib.rs-1277 */ impl<E: SpanEncoder> Encodable<E> for CrateNum {
/* FP:lib.rs-1278 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1279 */         s.encode_crate_num(*self)
/* FP:lib.rs-1280 */     }
/* FP:lib.rs-1281 */ }
/* FP:lib.rs-1282 */ 
/* FP:lib.rs-1283 */ impl<E: SpanEncoder> Encodable<E> for DefIndex {
/* FP:lib.rs-1284 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1285 */         s.encode_def_index(*self)
/* FP:lib.rs-1286 */     }
/* FP:lib.rs-1287 */ }
/* FP:lib.rs-1288 */ 
/* FP:lib.rs-1289 */ impl<E: SpanEncoder> Encodable<E> for DefId {
/* FP:lib.rs-1290 */     fn encode(&self, s: &mut E) {
/* FP:lib.rs-1291 */         s.encode_def_id(*self)
/* FP:lib.rs-1292 */     }
/* FP:lib.rs-1293 */ }
/* FP:lib.rs-1294 */ 
/* FP:lib.rs-1295 */ impl<E: SpanEncoder> Encodable<E> for AttrId {
/* FP:lib.rs-1296 */     fn encode(&self, _s: &mut E) {
/* FP:lib.rs-1297 */         // A fresh id will be generated when decoding
/* FP:lib.rs-1298 */     }
/* FP:lib.rs-1299 */ }
/* FP:lib.rs-1300 */ 
/* FP:lib.rs-1301 */ /// This trait is used to allow decoder specific encodings of certain types.
/* FP:lib.rs-1302 */ /// It is similar to rustc_type_ir's TyDecoder.
/* FP:lib.rs-1303 */ pub trait SpanDecoder: Decoder {
/* FP:lib.rs-1304 */     fn decode_span(&mut self) -> Span;
/* FP:lib.rs-1305 */     fn decode_symbol(&mut self) -> Symbol;
/* FP:lib.rs-1306 */     fn decode_byte_symbol(&mut self) -> ByteSymbol;
/* FP:lib.rs-1307 */     fn decode_expn_id(&mut self) -> ExpnId;
/* FP:lib.rs-1308 */     fn decode_syntax_context(&mut self) -> SyntaxContext;
/* FP:lib.rs-1309 */     fn decode_crate_num(&mut self) -> CrateNum;
/* FP:lib.rs-1310 */     fn decode_def_index(&mut self) -> DefIndex;
/* FP:lib.rs-1311 */     fn decode_def_id(&mut self) -> DefId;
/* FP:lib.rs-1312 */     fn decode_attr_id(&mut self) -> AttrId;
/* FP:lib.rs-1313 */ }
/* FP:lib.rs-1314 */ 
/* FP:lib.rs-1315 */ impl SpanDecoder for MemDecoder<'_> {
/* FP:lib.rs-1316 */     fn decode_span(&mut self) -> Span {
/* FP:lib.rs-1317 */         let lo = Decodable::decode(self);
/* FP:lib.rs-1318 */         let hi = Decodable::decode(self);
/* FP:lib.rs-1319 */ 
/* FP:lib.rs-1320 */         Span::new(lo, hi, SyntaxContext::root(), None)
/* FP:lib.rs-1321 */     }
/* FP:lib.rs-1322 */ 
/* FP:lib.rs-1323 */     fn decode_symbol(&mut self) -> Symbol {
/* FP:lib.rs-1324 */         Symbol::intern(self.read_str())
/* FP:lib.rs-1325 */     }
/* FP:lib.rs-1326 */ 
/* FP:lib.rs-1327 */     fn decode_byte_symbol(&mut self) -> ByteSymbol {
/* FP:lib.rs-1328 */         ByteSymbol::intern(self.read_byte_str())
/* FP:lib.rs-1329 */     }
/* FP:lib.rs-1330 */ 
/* FP:lib.rs-1331 */     fn decode_expn_id(&mut self) -> ExpnId {
/* FP:lib.rs-1332 */         panic!("cannot decode `ExpnId` with `MemDecoder`");
/* FP:lib.rs-1333 */     }
/* FP:lib.rs-1334 */ 
/* FP:lib.rs-1335 */     fn decode_syntax_context(&mut self) -> SyntaxContext {
/* FP:lib.rs-1336 */         panic!("cannot decode `SyntaxContext` with `MemDecoder`");
/* FP:lib.rs-1337 */     }
/* FP:lib.rs-1338 */ 
/* FP:lib.rs-1339 */     fn decode_crate_num(&mut self) -> CrateNum {
/* FP:lib.rs-1340 */         CrateNum::from_u32(self.read_u32())
/* FP:lib.rs-1341 */     }
/* FP:lib.rs-1342 */ 
/* FP:lib.rs-1343 */     fn decode_def_index(&mut self) -> DefIndex {
/* FP:lib.rs-1344 */         panic!("cannot decode `DefIndex` with `MemDecoder`");
/* FP:lib.rs-1345 */     }
/* FP:lib.rs-1346 */ 
/* FP:lib.rs-1347 */     fn decode_def_id(&mut self) -> DefId {
/* FP:lib.rs-1348 */         DefId { krate: Decodable::decode(self), index: Decodable::decode(self) }
/* FP:lib.rs-1349 */     }
/* FP:lib.rs-1350 */ 
/* FP:lib.rs-1351 */     fn decode_attr_id(&mut self) -> AttrId {
/* FP:lib.rs-1352 */         panic!("cannot decode `AttrId` with `MemDecoder`");
/* FP:lib.rs-1353 */     }
/* FP:lib.rs-1354 */ }
/* FP:lib.rs-1355 */ 
/* FP:lib.rs-1356 */ impl<D: SpanDecoder> Decodable<D> for Span {
/* FP:lib.rs-1357 */     fn decode(s: &mut D) -> Span {
/* FP:lib.rs-1358 */         s.decode_span()
/* FP:lib.rs-1359 */     }
/* FP:lib.rs-1360 */ }
/* FP:lib.rs-1361 */ 
/* FP:lib.rs-1362 */ impl<D: SpanDecoder> Decodable<D> for Symbol {
/* FP:lib.rs-1363 */     fn decode(s: &mut D) -> Symbol {
/* FP:lib.rs-1364 */         s.decode_symbol()
/* FP:lib.rs-1365 */     }
/* FP:lib.rs-1366 */ }
/* FP:lib.rs-1367 */ 
/* FP:lib.rs-1368 */ impl<D: SpanDecoder> Decodable<D> for ByteSymbol {
/* FP:lib.rs-1369 */     fn decode(s: &mut D) -> ByteSymbol {
/* FP:lib.rs-1370 */         s.decode_byte_symbol()
/* FP:lib.rs-1371 */     }
/* FP:lib.rs-1372 */ }
/* FP:lib.rs-1373 */ 
/* FP:lib.rs-1374 */ impl<D: SpanDecoder> Decodable<D> for ExpnId {
/* FP:lib.rs-1375 */     fn decode(s: &mut D) -> ExpnId {
/* FP:lib.rs-1376 */         s.decode_expn_id()
/* FP:lib.rs-1377 */     }
/* FP:lib.rs-1378 */ }
/* FP:lib.rs-1379 */ 
/* FP:lib.rs-1380 */ impl<D: SpanDecoder> Decodable<D> for SyntaxContext {
/* FP:lib.rs-1381 */     fn decode(s: &mut D) -> SyntaxContext {
/* FP:lib.rs-1382 */         s.decode_syntax_context()
/* FP:lib.rs-1383 */     }
/* FP:lib.rs-1384 */ }
/* FP:lib.rs-1385 */ 
/* FP:lib.rs-1386 */ impl<D: SpanDecoder> Decodable<D> for CrateNum {
/* FP:lib.rs-1387 */     fn decode(s: &mut D) -> CrateNum {
/* FP:lib.rs-1388 */         s.decode_crate_num()
/* FP:lib.rs-1389 */     }
/* FP:lib.rs-1390 */ }
/* FP:lib.rs-1391 */ 
/* FP:lib.rs-1392 */ impl<D: SpanDecoder> Decodable<D> for DefIndex {
/* FP:lib.rs-1393 */     fn decode(s: &mut D) -> DefIndex {
/* FP:lib.rs-1394 */         s.decode_def_index()
/* FP:lib.rs-1395 */     }
/* FP:lib.rs-1396 */ }
/* FP:lib.rs-1397 */ 
/* FP:lib.rs-1398 */ impl<D: SpanDecoder> Decodable<D> for DefId {
/* FP:lib.rs-1399 */     fn decode(s: &mut D) -> DefId {
/* FP:lib.rs-1400 */         s.decode_def_id()
/* FP:lib.rs-1401 */     }
/* FP:lib.rs-1402 */ }
/* FP:lib.rs-1403 */ 
/* FP:lib.rs-1404 */ impl<D: SpanDecoder> Decodable<D> for AttrId {
/* FP:lib.rs-1405 */     fn decode(s: &mut D) -> AttrId {
/* FP:lib.rs-1406 */         s.decode_attr_id()
/* FP:lib.rs-1407 */     }
/* FP:lib.rs-1408 */ }
/* FP:lib.rs-1409 */ 
/* FP:lib.rs-1410 */ impl fmt::Debug for Span {
/* FP:lib.rs-1411 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1412 */         // Use the global `SourceMap` to print the span. If that's not
/* FP:lib.rs-1413 */         // available, fall back to printing the raw values.
/* FP:lib.rs-1414 */ 
/* FP:lib.rs-1415 */         fn fallback(span: Span, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1416 */             f.debug_struct("Span")
/* FP:lib.rs-1417 */                 .field("lo", &span.lo())
/* FP:lib.rs-1418 */                 .field("hi", &span.hi())
/* FP:lib.rs-1419 */                 .field("ctxt", &span.ctxt())
/* FP:lib.rs-1420 */                 .finish()
/* FP:lib.rs-1421 */         }
/* FP:lib.rs-1422 */ 
/* FP:lib.rs-1423 */         if SESSION_GLOBALS.is_set() {
/* FP:lib.rs-1424 */             with_session_globals(|session_globals| {
/* FP:lib.rs-1425 */                 if let Some(source_map) = &session_globals.source_map {
/* FP:lib.rs-1426 */                     write!(f, "{} ({:?})", source_map.span_to_diagnostic_string(*self), self.ctxt())
/* FP:lib.rs-1427 */                 } else {
/* FP:lib.rs-1428 */                     fallback(*self, f)
/* FP:lib.rs-1429 */                 }
/* FP:lib.rs-1430 */             })
/* FP:lib.rs-1431 */         } else {
/* FP:lib.rs-1432 */             fallback(*self, f)
/* FP:lib.rs-1433 */         }
/* FP:lib.rs-1434 */     }
/* FP:lib.rs-1435 */ }
/* FP:lib.rs-1436 */ 
/* FP:lib.rs-1437 */ impl fmt::Debug for SpanData {
/* FP:lib.rs-1438 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1439 */         fmt::Debug::fmt(&self.span(), f)
/* FP:lib.rs-1440 */     }
/* FP:lib.rs-1441 */ }
/* FP:lib.rs-1442 */ 
/* FP:lib.rs-1443 */ /// Identifies an offset of a multi-byte character in a `SourceFile`.
/* FP:lib.rs-1444 */ #[derive(Copy, Clone, Encodable, Decodable, Eq, PartialEq, Debug, HashStable_Generic)]
/* FP:lib.rs-1445 */ pub struct MultiByteChar {
/* FP:lib.rs-1446 */     /// The relative offset of the character in the `SourceFile`.
/* FP:lib.rs-1447 */     pub pos: RelativeBytePos,
/* FP:lib.rs-1448 */     /// The number of bytes, `>= 2`.
/* FP:lib.rs-1449 */     pub bytes: u8,
/* FP:lib.rs-1450 */ }
/* FP:lib.rs-1451 */ 
/* FP:lib.rs-1452 */ /// Identifies an offset of a character that was normalized away from `SourceFile`.
/* FP:lib.rs-1453 */ #[derive(Copy, Clone, Encodable, Decodable, Eq, PartialEq, Debug, HashStable_Generic)]
/* FP:lib.rs-1454 */ pub struct NormalizedPos {
/* FP:lib.rs-1455 */     /// The relative offset of the character in the `SourceFile`.
/* FP:lib.rs-1456 */     pub pos: RelativeBytePos,
/* FP:lib.rs-1457 */     /// The difference between original and normalized string at position.
/* FP:lib.rs-1458 */     pub diff: u32,
/* FP:lib.rs-1459 */ }
/* FP:lib.rs-1460 */ 
/* FP:lib.rs-1461 */ #[derive(PartialEq, Eq, Clone, Debug)]
/* FP:lib.rs-1462 */ pub enum ExternalSource {
/* FP:lib.rs-1463 */     /// No external source has to be loaded, since the `SourceFile` represents a local crate.
/* FP:lib.rs-1464 */     Unneeded,
/* FP:lib.rs-1465 */     Foreign {
/* FP:lib.rs-1466 */         kind: ExternalSourceKind,
/* FP:lib.rs-1467 */         /// Index of the file inside metadata.
/* FP:lib.rs-1468 */         metadata_index: u32,
/* FP:lib.rs-1469 */     },
/* FP:lib.rs-1470 */ }
/* FP:lib.rs-1471 */ 
/* FP:lib.rs-1472 */ /// The state of the lazy external source loading mechanism of a `SourceFile`.
/* FP:lib.rs-1473 */ #[derive(PartialEq, Eq, Clone, Debug)]
/* FP:lib.rs-1474 */ pub enum ExternalSourceKind {
/* FP:lib.rs-1475 */     /// The external source has been loaded already.
/* FP:lib.rs-1476 */     Present(Arc<String>),
/* FP:lib.rs-1477 */     /// No attempt has been made to load the external source.
/* FP:lib.rs-1478 */     AbsentOk,
/* FP:lib.rs-1479 */     /// A failed attempt has been made to load the external source.
/* FP:lib.rs-1480 */     AbsentErr,
/* FP:lib.rs-1481 */ }
/* FP:lib.rs-1482 */ 
/* FP:lib.rs-1483 */ impl ExternalSource {
/* FP:lib.rs-1484 */     pub fn get_source(&self) -> Option<&str> {
/* FP:lib.rs-1485 */         match self {
/* FP:lib.rs-1486 */             ExternalSource::Foreign { kind: ExternalSourceKind::Present(src), .. } => Some(src),
/* FP:lib.rs-1487 */             _ => None,
/* FP:lib.rs-1488 */         }
/* FP:lib.rs-1489 */     }
/* FP:lib.rs-1490 */ }
/* FP:lib.rs-1491 */ 
/* FP:lib.rs-1492 */ #[derive(Debug)]
/* FP:lib.rs-1493 */ pub struct OffsetOverflowError;
/* FP:lib.rs-1494 */ 
/* FP:lib.rs-1495 */ #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Encodable, Decodable)]
/* FP:lib.rs-1496 */ #[derive(HashStable_Generic)]
/* FP:lib.rs-1497 */ pub enum SourceFileHashAlgorithm {
/* FP:lib.rs-1498 */     Md5,
/* FP:lib.rs-1499 */     Sha1,
/* FP:lib.rs-1500 */     Sha256,
/* FP:lib.rs-1501 */     Blake3,
/* FP:lib.rs-1502 */ }
/* FP:lib.rs-1503 */ 
/* FP:lib.rs-1504 */ impl Display for SourceFileHashAlgorithm {
/* FP:lib.rs-1505 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1506 */         f.write_str(match self {
/* FP:lib.rs-1507 */             Self::Md5 => "md5",
/* FP:lib.rs-1508 */             Self::Sha1 => "sha1",
/* FP:lib.rs-1509 */             Self::Sha256 => "sha256",
/* FP:lib.rs-1510 */             Self::Blake3 => "blake3",
/* FP:lib.rs-1511 */         })
/* FP:lib.rs-1512 */     }
/* FP:lib.rs-1513 */ }
/* FP:lib.rs-1514 */ 
/* FP:lib.rs-1515 */ impl FromStr for SourceFileHashAlgorithm {
/* FP:lib.rs-1516 */     type Err = ();
/* FP:lib.rs-1517 */ 
/* FP:lib.rs-1518 */     fn from_str(s: &str) -> Result<SourceFileHashAlgorithm, ()> {
/* FP:lib.rs-1519 */         match s {
/* FP:lib.rs-1520 */             "md5" => Ok(SourceFileHashAlgorithm::Md5),
/* FP:lib.rs-1521 */             "sha1" => Ok(SourceFileHashAlgorithm::Sha1),
/* FP:lib.rs-1522 */             "sha256" => Ok(SourceFileHashAlgorithm::Sha256),
/* FP:lib.rs-1523 */             "blake3" => Ok(SourceFileHashAlgorithm::Blake3),
/* FP:lib.rs-1524 */             _ => Err(()),
/* FP:lib.rs-1525 */         }
/* FP:lib.rs-1526 */     }
/* FP:lib.rs-1527 */ }
/* FP:lib.rs-1528 */ 
/* FP:lib.rs-1529 */ /// The hash of the on-disk source file used for debug info and cargo freshness checks.
/* FP:lib.rs-1530 */ #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
/* FP:lib.rs-1531 */ #[derive(HashStable_Generic, Encodable, Decodable)]
/* FP:lib.rs-1532 */ pub struct SourceFileHash {
/* FP:lib.rs-1533 */     pub kind: SourceFileHashAlgorithm,
/* FP:lib.rs-1534 */     value: [u8; 32],
/* FP:lib.rs-1535 */ }
/* FP:lib.rs-1536 */ 
/* FP:lib.rs-1537 */ impl Display for SourceFileHash {
/* FP:lib.rs-1538 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1539 */         write!(f, "{}=", self.kind)?;
/* FP:lib.rs-1540 */         for byte in self.value[0..self.hash_len()].into_iter() {
/* FP:lib.rs-1541 */             write!(f, "{byte:02x}")?;
/* FP:lib.rs-1542 */         }
/* FP:lib.rs-1543 */         Ok(())
/* FP:lib.rs-1544 */     }
/* FP:lib.rs-1545 */ }
/* FP:lib.rs-1546 */ 
/* FP:lib.rs-1547 */ impl SourceFileHash {
/* FP:lib.rs-1548 */     pub fn new_in_memory(kind: SourceFileHashAlgorithm, src: impl AsRef<[u8]>) -> SourceFileHash {
/* FP:lib.rs-1549 */         let mut hash = SourceFileHash { kind, value: Default::default() };
/* FP:lib.rs-1550 */         let len = hash.hash_len();
/* FP:lib.rs-1551 */         let value = &mut hash.value[..len];
/* FP:lib.rs-1552 */         let data = src.as_ref();
/* FP:lib.rs-1553 */         match kind {
/* FP:lib.rs-1554 */             SourceFileHashAlgorithm::Md5 => {
/* FP:lib.rs-1555 */                 value.copy_from_slice(&Md5::digest(data));
/* FP:lib.rs-1556 */             }
/* FP:lib.rs-1557 */             SourceFileHashAlgorithm::Sha1 => {
/* FP:lib.rs-1558 */                 value.copy_from_slice(&Sha1::digest(data));
/* FP:lib.rs-1559 */             }
/* FP:lib.rs-1560 */             SourceFileHashAlgorithm::Sha256 => {
/* FP:lib.rs-1561 */                 value.copy_from_slice(&Sha256::digest(data));
/* FP:lib.rs-1562 */             }
/* FP:lib.rs-1563 */             SourceFileHashAlgorithm::Blake3 => value.copy_from_slice(blake3::hash(data).as_bytes()),
/* FP:lib.rs-1564 */         };
/* FP:lib.rs-1565 */         hash
/* FP:lib.rs-1566 */     }
/* FP:lib.rs-1567 */ 
/* FP:lib.rs-1568 */     pub fn new(kind: SourceFileHashAlgorithm, src: impl Read) -> Result<SourceFileHash, io::Error> {
/* FP:lib.rs-1569 */         let mut hash = SourceFileHash { kind, value: Default::default() };
/* FP:lib.rs-1570 */         let len = hash.hash_len();
/* FP:lib.rs-1571 */         let value = &mut hash.value[..len];
/* FP:lib.rs-1572 */         // Buffer size is the recommended amount to fully leverage SIMD instructions on AVX-512 as per
/* FP:lib.rs-1573 */         // blake3 documentation.
/* FP:lib.rs-1574 */         let mut buf = vec![0; 16 * 1024];
/* FP:lib.rs-1575 */ 
/* FP:lib.rs-1576 */         fn digest<T>(
/* FP:lib.rs-1577 */             mut hasher: T,
/* FP:lib.rs-1578 */             mut update: impl FnMut(&mut T, &[u8]),
/* FP:lib.rs-1579 */             finish: impl FnOnce(T, &mut [u8]),
/* FP:lib.rs-1580 */             mut src: impl Read,
/* FP:lib.rs-1581 */             buf: &mut [u8],
/* FP:lib.rs-1582 */             value: &mut [u8],
/* FP:lib.rs-1583 */         ) -> Result<(), io::Error> {
/* FP:lib.rs-1584 */             loop {
/* FP:lib.rs-1585 */                 let bytes_read = src.read(buf)?;
/* FP:lib.rs-1586 */                 if bytes_read == 0 {
/* FP:lib.rs-1587 */                     break;
/* FP:lib.rs-1588 */                 }
/* FP:lib.rs-1589 */                 update(&mut hasher, &buf[0..bytes_read]);
/* FP:lib.rs-1590 */             }
/* FP:lib.rs-1591 */             finish(hasher, value);
/* FP:lib.rs-1592 */             Ok(())
/* FP:lib.rs-1593 */         }
/* FP:lib.rs-1594 */ 
/* FP:lib.rs-1595 */         match kind {
/* FP:lib.rs-1596 */             SourceFileHashAlgorithm::Sha256 => {
/* FP:lib.rs-1597 */                 digest(
/* FP:lib.rs-1598 */                     Sha256::new(),
/* FP:lib.rs-1599 */                     |h, b| {
/* FP:lib.rs-1600 */                         h.update(b);
/* FP:lib.rs-1601 */                     },
/* FP:lib.rs-1602 */                     |h, out| out.copy_from_slice(&h.finalize()),
/* FP:lib.rs-1603 */                     src,
/* FP:lib.rs-1604 */                     &mut buf,
/* FP:lib.rs-1605 */                     value,
/* FP:lib.rs-1606 */                 )?;
/* FP:lib.rs-1607 */             }
/* FP:lib.rs-1608 */             SourceFileHashAlgorithm::Sha1 => {
/* FP:lib.rs-1609 */                 digest(
/* FP:lib.rs-1610 */                     Sha1::new(),
/* FP:lib.rs-1611 */                     |h, b| {
/* FP:lib.rs-1612 */                         h.update(b);
/* FP:lib.rs-1613 */                     },
/* FP:lib.rs-1614 */                     |h, out| out.copy_from_slice(&h.finalize()),
/* FP:lib.rs-1615 */                     src,
/* FP:lib.rs-1616 */                     &mut buf,
/* FP:lib.rs-1617 */                     value,
/* FP:lib.rs-1618 */                 )?;
/* FP:lib.rs-1619 */             }
/* FP:lib.rs-1620 */             SourceFileHashAlgorithm::Md5 => {
/* FP:lib.rs-1621 */                 digest(
/* FP:lib.rs-1622 */                     Md5::new(),
/* FP:lib.rs-1623 */                     |h, b| {
/* FP:lib.rs-1624 */                         h.update(b);
/* FP:lib.rs-1625 */                     },
/* FP:lib.rs-1626 */                     |h, out| out.copy_from_slice(&h.finalize()),
/* FP:lib.rs-1627 */                     src,
/* FP:lib.rs-1628 */                     &mut buf,
/* FP:lib.rs-1629 */                     value,
/* FP:lib.rs-1630 */                 )?;
/* FP:lib.rs-1631 */             }
/* FP:lib.rs-1632 */             SourceFileHashAlgorithm::Blake3 => {
/* FP:lib.rs-1633 */                 digest(
/* FP:lib.rs-1634 */                     blake3::Hasher::new(),
/* FP:lib.rs-1635 */                     |h, b| {
/* FP:lib.rs-1636 */                         h.update(b);
/* FP:lib.rs-1637 */                     },
/* FP:lib.rs-1638 */                     |h, out| out.copy_from_slice(h.finalize().as_bytes()),
/* FP:lib.rs-1639 */                     src,
/* FP:lib.rs-1640 */                     &mut buf,
/* FP:lib.rs-1641 */                     value,
/* FP:lib.rs-1642 */                 )?;
/* FP:lib.rs-1643 */             }
/* FP:lib.rs-1644 */         }
/* FP:lib.rs-1645 */         Ok(hash)
/* FP:lib.rs-1646 */     }
/* FP:lib.rs-1647 */ 
/* FP:lib.rs-1648 */     /// Check if the stored hash matches the hash of the string.
/* FP:lib.rs-1649 */     pub fn matches(&self, src: &str) -> bool {
/* FP:lib.rs-1650 */         Self::new_in_memory(self.kind, src.as_bytes()) == *self
/* FP:lib.rs-1651 */     }
/* FP:lib.rs-1652 */ 
/* FP:lib.rs-1653 */     /// The bytes of the hash.
/* FP:lib.rs-1654 */     pub fn hash_bytes(&self) -> &[u8] {
/* FP:lib.rs-1655 */         let len = self.hash_len();
/* FP:lib.rs-1656 */         &self.value[..len]
/* FP:lib.rs-1657 */     }
/* FP:lib.rs-1658 */ 
/* FP:lib.rs-1659 */     fn hash_len(&self) -> usize {
/* FP:lib.rs-1660 */         match self.kind {
/* FP:lib.rs-1661 */             SourceFileHashAlgorithm::Md5 => 16,
/* FP:lib.rs-1662 */             SourceFileHashAlgorithm::Sha1 => 20,
/* FP:lib.rs-1663 */             SourceFileHashAlgorithm::Sha256 | SourceFileHashAlgorithm::Blake3 => 32,
/* FP:lib.rs-1664 */         }
/* FP:lib.rs-1665 */     }
/* FP:lib.rs-1666 */ }
/* FP:lib.rs-1667 */ 
/* FP:lib.rs-1668 */ #[derive(Clone)]
/* FP:lib.rs-1669 */ pub enum SourceFileLines {
/* FP:lib.rs-1670 */     /// The source file lines, in decoded (random-access) form.
/* FP:lib.rs-1671 */     Lines(Vec<RelativeBytePos>),
/* FP:lib.rs-1672 */ 
/* FP:lib.rs-1673 */     /// The source file lines, in undecoded difference list form.
/* FP:lib.rs-1674 */     Diffs(SourceFileDiffs),
/* FP:lib.rs-1675 */ }
/* FP:lib.rs-1676 */ 
/* FP:lib.rs-1677 */ impl SourceFileLines {
/* FP:lib.rs-1678 */     pub fn is_lines(&self) -> bool {
/* FP:lib.rs-1679 */         matches!(self, SourceFileLines::Lines(_))
/* FP:lib.rs-1680 */     }
/* FP:lib.rs-1681 */ }
/* FP:lib.rs-1682 */ 
/* FP:lib.rs-1683 */ /// The source file lines in difference list form. This matches the form
/* FP:lib.rs-1684 */ /// used within metadata, which saves space by exploiting the fact that the
/* FP:lib.rs-1685 */ /// lines list is sorted and individual lines are usually not that long.
/* FP:lib.rs-1686 */ ///
/* FP:lib.rs-1687 */ /// We read it directly from metadata and only decode it into `Lines` form
/* FP:lib.rs-1688 */ /// when necessary. This is a significant performance win, especially for
/* FP:lib.rs-1689 */ /// small crates where very little of `std`'s metadata is used.
/* FP:lib.rs-1690 */ #[derive(Clone)]
/* FP:lib.rs-1691 */ pub struct SourceFileDiffs {
/* FP:lib.rs-1692 */     /// Always 1, 2, or 4. Always as small as possible, while being big
/* FP:lib.rs-1693 */     /// enough to hold the length of the longest line in the source file.
/* FP:lib.rs-1694 */     /// The 1 case is by far the most common.
/* FP:lib.rs-1695 */     bytes_per_diff: usize,
/* FP:lib.rs-1696 */ 
/* FP:lib.rs-1697 */     /// The number of diffs encoded in `raw_diffs`. Always one less than
/* FP:lib.rs-1698 */     /// the number of lines in the source file.
/* FP:lib.rs-1699 */     num_diffs: usize,
/* FP:lib.rs-1700 */ 
/* FP:lib.rs-1701 */     /// The diffs in "raw" form. Each segment of `bytes_per_diff` length
/* FP:lib.rs-1702 */     /// encodes one little-endian diff. Note that they aren't LEB128
/* FP:lib.rs-1703 */     /// encoded. This makes for much faster decoding. Besides, the
/* FP:lib.rs-1704 */     /// bytes_per_diff==1 case is by far the most common, and LEB128
/* FP:lib.rs-1705 */     /// encoding has no effect on that case.
/* FP:lib.rs-1706 */     raw_diffs: Vec<u8>,
/* FP:lib.rs-1707 */ }
/* FP:lib.rs-1708 */ 
/* FP:lib.rs-1709 */ /// A single source in the [`SourceMap`].
/* FP:lib.rs-1710 */ pub struct SourceFile {
/* FP:lib.rs-1711 */     /// The name of the file that the source came from. Source that doesn't
/* FP:lib.rs-1712 */     /// originate from files has names between angle brackets by convention
/* FP:lib.rs-1713 */     /// (e.g., `<anon>`).
/* FP:lib.rs-1714 */     pub name: FileName,
/* FP:lib.rs-1715 */     /// The complete source code.
/* FP:lib.rs-1716 */     pub src: Option<Arc<String>>,
/* FP:lib.rs-1717 */     /// The source code's hash.
/* FP:lib.rs-1718 */     pub src_hash: SourceFileHash,
/* FP:lib.rs-1719 */     /// Used to enable cargo to use checksums to check if a crate is fresh rather
/* FP:lib.rs-1720 */     /// than mtimes. This might be the same as `src_hash`, and if the requested algorithm
/* FP:lib.rs-1721 */     /// is identical we won't compute it twice.
/* FP:lib.rs-1722 */     pub checksum_hash: Option<SourceFileHash>,
/* FP:lib.rs-1723 */     /// The external source code (used for external crates, which will have a `None`
/* FP:lib.rs-1724 */     /// value as `self.src`.
/* FP:lib.rs-1725 */     pub external_src: FreezeLock<ExternalSource>,
/* FP:lib.rs-1726 */     /// The start position of this source in the `SourceMap`.
/* FP:lib.rs-1727 */     pub start_pos: BytePos,
/* FP:lib.rs-1728 */     /// The byte length of this source.
/* FP:lib.rs-1729 */     pub source_len: RelativeBytePos,
/* FP:lib.rs-1730 */     /// Locations of lines beginnings in the source code.
/* FP:lib.rs-1731 */     pub lines: FreezeLock<SourceFileLines>,
/* FP:lib.rs-1732 */     /// Locations of multi-byte characters in the source code.
/* FP:lib.rs-1733 */     pub multibyte_chars: Vec<MultiByteChar>,
/* FP:lib.rs-1734 */     /// Locations of characters removed during normalization.
/* FP:lib.rs-1735 */     pub normalized_pos: Vec<NormalizedPos>,
/* FP:lib.rs-1736 */     /// A hash of the filename & crate-id, used for uniquely identifying source
/* FP:lib.rs-1737 */     /// files within the crate graph and for speeding up hashing in incremental
/* FP:lib.rs-1738 */     /// compilation.
/* FP:lib.rs-1739 */     pub stable_id: StableSourceFileId,
/* FP:lib.rs-1740 */     /// Indicates which crate this `SourceFile` was imported from.
/* FP:lib.rs-1741 */     pub cnum: CrateNum,
/* FP:lib.rs-1742 */ }
/* FP:lib.rs-1743 */ 
/* FP:lib.rs-1744 */ impl Clone for SourceFile {
/* FP:lib.rs-1745 */     fn clone(&self) -> Self {
/* FP:lib.rs-1746 */         Self {
/* FP:lib.rs-1747 */             name: self.name.clone(),
/* FP:lib.rs-1748 */             src: self.src.clone(),
/* FP:lib.rs-1749 */             src_hash: self.src_hash,
/* FP:lib.rs-1750 */             checksum_hash: self.checksum_hash,
/* FP:lib.rs-1751 */             external_src: self.external_src.clone(),
/* FP:lib.rs-1752 */             start_pos: self.start_pos,
/* FP:lib.rs-1753 */             source_len: self.source_len,
/* FP:lib.rs-1754 */             lines: self.lines.clone(),
/* FP:lib.rs-1755 */             multibyte_chars: self.multibyte_chars.clone(),
/* FP:lib.rs-1756 */             normalized_pos: self.normalized_pos.clone(),
/* FP:lib.rs-1757 */             stable_id: self.stable_id,
/* FP:lib.rs-1758 */             cnum: self.cnum,
/* FP:lib.rs-1759 */         }
/* FP:lib.rs-1760 */     }
/* FP:lib.rs-1761 */ }
/* FP:lib.rs-1762 */ 
/* FP:lib.rs-1763 */ impl<S: SpanEncoder> Encodable<S> for SourceFile {
/* FP:lib.rs-1764 */     fn encode(&self, s: &mut S) {
/* FP:lib.rs-1765 */         self.name.encode(s);
/* FP:lib.rs-1766 */         self.src_hash.encode(s);
/* FP:lib.rs-1767 */         self.checksum_hash.encode(s);
/* FP:lib.rs-1768 */         // Do not encode `start_pos` as it's global state for this session.
/* FP:lib.rs-1769 */         self.source_len.encode(s);
/* FP:lib.rs-1770 */ 
/* FP:lib.rs-1771 */         // We are always in `Lines` form by the time we reach here.
/* FP:lib.rs-1772 */         assert!(self.lines.read().is_lines());
/* FP:lib.rs-1773 */         let lines = self.lines();
/* FP:lib.rs-1774 */         // Store the length.
/* FP:lib.rs-1775 */         s.emit_u32(lines.len() as u32);
/* FP:lib.rs-1776 */ 
/* FP:lib.rs-1777 */         // Compute and store the difference list.
/* FP:lib.rs-1778 */         if lines.len() != 0 {
/* FP:lib.rs-1779 */             let max_line_length = if lines.len() == 1 {
/* FP:lib.rs-1780 */                 0
/* FP:lib.rs-1781 */             } else {
/* FP:lib.rs-1782 */                 lines
/* FP:lib.rs-1783 */                     .array_windows()
/* FP:lib.rs-1784 */                     .map(|&[fst, snd]| snd - fst)
/* FP:lib.rs-1785 */                     .map(|bp| bp.to_usize())
/* FP:lib.rs-1786 */                     .max()
/* FP:lib.rs-1787 */                     .unwrap()
/* FP:lib.rs-1788 */             };
/* FP:lib.rs-1789 */ 
/* FP:lib.rs-1790 */             let bytes_per_diff: usize = match max_line_length {
/* FP:lib.rs-1791 */                 0..=0xFF => 1,
/* FP:lib.rs-1792 */                 0x100..=0xFFFF => 2,
/* FP:lib.rs-1793 */                 _ => 4,
/* FP:lib.rs-1794 */             };
/* FP:lib.rs-1795 */ 
/* FP:lib.rs-1796 */             // Encode the number of bytes used per diff.
/* FP:lib.rs-1797 */             s.emit_u8(bytes_per_diff as u8);
/* FP:lib.rs-1798 */ 
/* FP:lib.rs-1799 */             // Encode the first element.
/* FP:lib.rs-1800 */             assert_eq!(lines[0], RelativeBytePos(0));
/* FP:lib.rs-1801 */ 
/* FP:lib.rs-1802 */             // Encode the difference list.
/* FP:lib.rs-1803 */             let diff_iter = lines.array_windows().map(|&[fst, snd]| snd - fst);
/* FP:lib.rs-1804 */             let num_diffs = lines.len() - 1;
/* FP:lib.rs-1805 */             let mut raw_diffs;
/* FP:lib.rs-1806 */             match bytes_per_diff {
/* FP:lib.rs-1807 */                 1 => {
/* FP:lib.rs-1808 */                     raw_diffs = Vec::with_capacity(num_diffs);
/* FP:lib.rs-1809 */                     for diff in diff_iter {
/* FP:lib.rs-1810 */                         raw_diffs.push(diff.0 as u8);
/* FP:lib.rs-1811 */                     }
/* FP:lib.rs-1812 */                 }
/* FP:lib.rs-1813 */                 2 => {
/* FP:lib.rs-1814 */                     raw_diffs = Vec::with_capacity(bytes_per_diff * num_diffs);
/* FP:lib.rs-1815 */                     for diff in diff_iter {
/* FP:lib.rs-1816 */                         raw_diffs.extend_from_slice(&(diff.0 as u16).to_le_bytes());
/* FP:lib.rs-1817 */                     }
/* FP:lib.rs-1818 */                 }
/* FP:lib.rs-1819 */                 4 => {
/* FP:lib.rs-1820 */                     raw_diffs = Vec::with_capacity(bytes_per_diff * num_diffs);
/* FP:lib.rs-1821 */                     for diff in diff_iter {
/* FP:lib.rs-1822 */                         raw_diffs.extend_from_slice(&(diff.0).to_le_bytes());
/* FP:lib.rs-1823 */                     }
/* FP:lib.rs-1824 */                 }
/* FP:lib.rs-1825 */                 _ => unreachable!(),
/* FP:lib.rs-1826 */             }
/* FP:lib.rs-1827 */             s.emit_raw_bytes(&raw_diffs);
/* FP:lib.rs-1828 */         }
/* FP:lib.rs-1829 */ 
/* FP:lib.rs-1830 */         self.multibyte_chars.encode(s);
/* FP:lib.rs-1831 */         self.stable_id.encode(s);
/* FP:lib.rs-1832 */         self.normalized_pos.encode(s);
/* FP:lib.rs-1833 */         self.cnum.encode(s);
/* FP:lib.rs-1834 */     }
/* FP:lib.rs-1835 */ }
/* FP:lib.rs-1836 */ 
/* FP:lib.rs-1837 */ impl<D: SpanDecoder> Decodable<D> for SourceFile {
/* FP:lib.rs-1838 */     fn decode(d: &mut D) -> SourceFile {
/* FP:lib.rs-1839 */         let name: FileName = Decodable::decode(d);
/* FP:lib.rs-1840 */         let src_hash: SourceFileHash = Decodable::decode(d);
/* FP:lib.rs-1841 */         let checksum_hash: Option<SourceFileHash> = Decodable::decode(d);
/* FP:lib.rs-1842 */         let source_len: RelativeBytePos = Decodable::decode(d);
/* FP:lib.rs-1843 */         let lines = {
/* FP:lib.rs-1844 */             let num_lines: u32 = Decodable::decode(d);
/* FP:lib.rs-1845 */             if num_lines > 0 {
/* FP:lib.rs-1846 */                 // Read the number of bytes used per diff.
/* FP:lib.rs-1847 */                 let bytes_per_diff = d.read_u8() as usize;
/* FP:lib.rs-1848 */ 
/* FP:lib.rs-1849 */                 // Read the difference list.
/* FP:lib.rs-1850 */                 let num_diffs = num_lines as usize - 1;
/* FP:lib.rs-1851 */                 let raw_diffs = d.read_raw_bytes(bytes_per_diff * num_diffs).to_vec();
/* FP:lib.rs-1852 */                 SourceFileLines::Diffs(SourceFileDiffs { bytes_per_diff, num_diffs, raw_diffs })
/* FP:lib.rs-1853 */             } else {
/* FP:lib.rs-1854 */                 SourceFileLines::Lines(vec![])
/* FP:lib.rs-1855 */             }
/* FP:lib.rs-1856 */         };
/* FP:lib.rs-1857 */         let multibyte_chars: Vec<MultiByteChar> = Decodable::decode(d);
/* FP:lib.rs-1858 */         let stable_id = Decodable::decode(d);
/* FP:lib.rs-1859 */         let normalized_pos: Vec<NormalizedPos> = Decodable::decode(d);
/* FP:lib.rs-1860 */         let cnum: CrateNum = Decodable::decode(d);
/* FP:lib.rs-1861 */         SourceFile {
/* FP:lib.rs-1862 */             name,
/* FP:lib.rs-1863 */             start_pos: BytePos::from_u32(0),
/* FP:lib.rs-1864 */             source_len,
/* FP:lib.rs-1865 */             src: None,
/* FP:lib.rs-1866 */             src_hash,
/* FP:lib.rs-1867 */             checksum_hash,
/* FP:lib.rs-1868 */             // Unused - the metadata decoder will construct
/* FP:lib.rs-1869 */             // a new SourceFile, filling in `external_src` properly
/* FP:lib.rs-1870 */             external_src: FreezeLock::frozen(ExternalSource::Unneeded),
/* FP:lib.rs-1871 */             lines: FreezeLock::new(lines),
/* FP:lib.rs-1872 */             multibyte_chars,
/* FP:lib.rs-1873 */             normalized_pos,
/* FP:lib.rs-1874 */             stable_id,
/* FP:lib.rs-1875 */             cnum,
/* FP:lib.rs-1876 */         }
/* FP:lib.rs-1877 */     }
/* FP:lib.rs-1878 */ }
/* FP:lib.rs-1879 */ 
/* FP:lib.rs-1880 */ impl fmt::Debug for SourceFile {
/* FP:lib.rs-1881 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1882 */         write!(fmt, "SourceFile({:?})", self.name)
/* FP:lib.rs-1883 */     }
/* FP:lib.rs-1884 */ }
/* FP:lib.rs-1885 */ 
/* FP:lib.rs-1886 */ /// This is a [SourceFile] identifier that is used to correlate source files between
/* FP:lib.rs-1887 */ /// subsequent compilation sessions (which is something we need to do during
/* FP:lib.rs-1888 */ /// incremental compilation).
/* FP:lib.rs-1889 */ ///
/* FP:lib.rs-1890 */ /// It is a hash value (so we can efficiently consume it when stable-hashing
/* FP:lib.rs-1891 */ /// spans) that consists of the `FileName` and the `StableCrateId` of the crate
/* FP:lib.rs-1892 */ /// the source file is from. The crate id is needed because sometimes the
/* FP:lib.rs-1893 */ /// `FileName` is not unique within the crate graph (think `src/lib.rs`, for
/* FP:lib.rs-1894 */ /// example).
/* FP:lib.rs-1895 */ ///
/* FP:lib.rs-1896 */ /// The way the crate-id part is handled is a bit special: source files of the
/* FP:lib.rs-1897 */ /// local crate are hashed as `(filename, None)`, while source files from
/* FP:lib.rs-1898 */ /// upstream crates have a hash of `(filename, Some(stable_crate_id))`. This
/* FP:lib.rs-1899 */ /// is because SourceFiles for the local crate are allocated very early in the
/* FP:lib.rs-1900 */ /// compilation process when the `StableCrateId` is not yet known. If, due to
/* FP:lib.rs-1901 */ /// some refactoring of the compiler, the `StableCrateId` of the local crate
/* FP:lib.rs-1902 */ /// were to become available, it would be better to uniformly make this a
/* FP:lib.rs-1903 */ /// hash of `(filename, stable_crate_id)`.
/* FP:lib.rs-1904 */ ///
/* FP:lib.rs-1905 */ /// When `SourceFile`s are exported in crate metadata, the `StableSourceFileId`
/* FP:lib.rs-1906 */ /// is updated to incorporate the `StableCrateId` of the exporting crate.
/* FP:lib.rs-1907 */ #[derive(
/* FP:lib.rs-1908 */     Debug,
/* FP:lib.rs-1909 */     Clone,
/* FP:lib.rs-1910 */     Copy,
/* FP:lib.rs-1911 */     Hash,
/* FP:lib.rs-1912 */     PartialEq,
/* FP:lib.rs-1913 */     Eq,
/* FP:lib.rs-1914 */     HashStable_Generic,
/* FP:lib.rs-1915 */     Encodable,
/* FP:lib.rs-1916 */     Decodable,
/* FP:lib.rs-1917 */     Default,
/* FP:lib.rs-1918 */     PartialOrd,
/* FP:lib.rs-1919 */     Ord
/* FP:lib.rs-1920 */ )]
/* FP:lib.rs-1921 */ pub struct StableSourceFileId(Hash128);
/* FP:lib.rs-1922 */ 
/* FP:lib.rs-1923 */ impl StableSourceFileId {
/* FP:lib.rs-1924 */     fn from_filename_in_current_crate(filename: &FileName) -> Self {
/* FP:lib.rs-1925 */         Self::from_filename_and_stable_crate_id(filename, None)
/* FP:lib.rs-1926 */     }
/* FP:lib.rs-1927 */ 
/* FP:lib.rs-1928 */     pub fn from_filename_for_export(
/* FP:lib.rs-1929 */         filename: &FileName,
/* FP:lib.rs-1930 */         local_crate_stable_crate_id: StableCrateId,
/* FP:lib.rs-1931 */     ) -> Self {
/* FP:lib.rs-1932 */         Self::from_filename_and_stable_crate_id(filename, Some(local_crate_stable_crate_id))
/* FP:lib.rs-1933 */     }
/* FP:lib.rs-1934 */ 
/* FP:lib.rs-1935 */     fn from_filename_and_stable_crate_id(
/* FP:lib.rs-1936 */         filename: &FileName,
/* FP:lib.rs-1937 */         stable_crate_id: Option<StableCrateId>,
/* FP:lib.rs-1938 */     ) -> Self {
/* FP:lib.rs-1939 */         let mut hasher = StableHasher::new();
/* FP:lib.rs-1940 */         filename.hash(&mut hasher);
/* FP:lib.rs-1941 */         stable_crate_id.hash(&mut hasher);
/* FP:lib.rs-1942 */         StableSourceFileId(hasher.finish())
/* FP:lib.rs-1943 */     }
/* FP:lib.rs-1944 */ }
/* FP:lib.rs-1945 */ 
/* FP:lib.rs-1946 */ impl SourceFile {
/* FP:lib.rs-1947 */     const MAX_FILE_SIZE: u32 = u32::MAX - 1;
/* FP:lib.rs-1948 */ 
/* FP:lib.rs-1949 */     pub fn new(
/* FP:lib.rs-1950 */         name: FileName,
/* FP:lib.rs-1951 */         mut src: String,
/* FP:lib.rs-1952 */         hash_kind: SourceFileHashAlgorithm,
/* FP:lib.rs-1953 */         checksum_hash_kind: Option<SourceFileHashAlgorithm>,
/* FP:lib.rs-1954 */     ) -> Result<Self, OffsetOverflowError> {
/* FP:lib.rs-1955 */         // Compute the file hash before any normalization.
/* FP:lib.rs-1956 */         let src_hash = SourceFileHash::new_in_memory(hash_kind, src.as_bytes());
/* FP:lib.rs-1957 */         let checksum_hash = checksum_hash_kind.map(|checksum_hash_kind| {
/* FP:lib.rs-1958 */             if checksum_hash_kind == hash_kind {
/* FP:lib.rs-1959 */                 src_hash
/* FP:lib.rs-1960 */             } else {
/* FP:lib.rs-1961 */                 SourceFileHash::new_in_memory(checksum_hash_kind, src.as_bytes())
/* FP:lib.rs-1962 */             }
/* FP:lib.rs-1963 */         });
/* FP:lib.rs-1964 */         let normalized_pos = normalize_src(&mut src);
/* FP:lib.rs-1965 */ 
/* FP:lib.rs-1966 */         let stable_id = StableSourceFileId::from_filename_in_current_crate(&name);
/* FP:lib.rs-1967 */         let source_len = src.len();
/* FP:lib.rs-1968 */         let source_len = u32::try_from(source_len).map_err(|_| OffsetOverflowError)?;
/* FP:lib.rs-1969 */         if source_len > Self::MAX_FILE_SIZE {
/* FP:lib.rs-1970 */             return Err(OffsetOverflowError);
/* FP:lib.rs-1971 */         }
/* FP:lib.rs-1972 */ 
/* FP:lib.rs-1973 */         let (lines, multibyte_chars) = analyze_source_file::analyze_source_file(&src);
/* FP:lib.rs-1974 */ 
/* FP:lib.rs-1975 */         Ok(SourceFile {
/* FP:lib.rs-1976 */             name,
/* FP:lib.rs-1977 */             src: Some(Arc::new(src)),
/* FP:lib.rs-1978 */             src_hash,
/* FP:lib.rs-1979 */             checksum_hash,
/* FP:lib.rs-1980 */             external_src: FreezeLock::frozen(ExternalSource::Unneeded),
/* FP:lib.rs-1981 */             start_pos: BytePos::from_u32(0),
/* FP:lib.rs-1982 */             source_len: RelativeBytePos::from_u32(source_len),
/* FP:lib.rs-1983 */             lines: FreezeLock::frozen(SourceFileLines::Lines(lines)),
/* FP:lib.rs-1984 */             multibyte_chars,
/* FP:lib.rs-1985 */             normalized_pos,
/* FP:lib.rs-1986 */             stable_id,
/* FP:lib.rs-1987 */             cnum: LOCAL_CRATE,
/* FP:lib.rs-1988 */         })
/* FP:lib.rs-1989 */     }
/* FP:lib.rs-1990 */ 
/* FP:lib.rs-1991 */     /// This converts the `lines` field to contain `SourceFileLines::Lines` if needed and freezes
/* FP:lib.rs-1992 */     /// it.
/* FP:lib.rs-1993 */     fn convert_diffs_to_lines_frozen(&self) {
/* FP:lib.rs-1994 */         let mut guard = if let Some(guard) = self.lines.try_write() { guard } else { return };
/* FP:lib.rs-1995 */ 
/* FP:lib.rs-1996 */         let SourceFileDiffs { bytes_per_diff, num_diffs, raw_diffs } = match &*guard {
/* FP:lib.rs-1997 */             SourceFileLines::Diffs(diffs) => diffs,
/* FP:lib.rs-1998 */             SourceFileLines::Lines(..) => {
/* FP:lib.rs-1999 */                 FreezeWriteGuard::freeze(guard);
/* FP:lib.rs-2000 */                 return;
/* FP:lib.rs-2001 */             }
/* FP:lib.rs-2002 */         };
/* FP:lib.rs-2003 */ 
/* FP:lib.rs-2004 */         // Convert from "diffs" form to "lines" form.
/* FP:lib.rs-2005 */         let num_lines = num_diffs + 1;
/* FP:lib.rs-2006 */         let mut lines = Vec::with_capacity(num_lines);
/* FP:lib.rs-2007 */         let mut line_start = RelativeBytePos(0);
/* FP:lib.rs-2008 */         lines.push(line_start);
/* FP:lib.rs-2009 */ 
/* FP:lib.rs-2010 */         assert_eq!(*num_diffs, raw_diffs.len() / bytes_per_diff);
/* FP:lib.rs-2011 */         match bytes_per_diff {
/* FP:lib.rs-2012 */             1 => {
/* FP:lib.rs-2013 */                 lines.extend(raw_diffs.into_iter().map(|&diff| {
/* FP:lib.rs-2014 */                     line_start = line_start + RelativeBytePos(diff as u32);
/* FP:lib.rs-2015 */                     line_start
/* FP:lib.rs-2016 */                 }));
/* FP:lib.rs-2017 */             }
/* FP:lib.rs-2018 */             2 => {
/* FP:lib.rs-2019 */                 lines.extend((0..*num_diffs).map(|i| {
/* FP:lib.rs-2020 */                     let pos = bytes_per_diff * i;
/* FP:lib.rs-2021 */                     let bytes = [raw_diffs[pos], raw_diffs[pos + 1]];
/* FP:lib.rs-2022 */                     let diff = u16::from_le_bytes(bytes);
/* FP:lib.rs-2023 */                     line_start = line_start + RelativeBytePos(diff as u32);
/* FP:lib.rs-2024 */                     line_start
/* FP:lib.rs-2025 */                 }));
/* FP:lib.rs-2026 */             }
/* FP:lib.rs-2027 */             4 => {
/* FP:lib.rs-2028 */                 lines.extend((0..*num_diffs).map(|i| {
/* FP:lib.rs-2029 */                     let pos = bytes_per_diff * i;
/* FP:lib.rs-2030 */                     let bytes = [
/* FP:lib.rs-2031 */                         raw_diffs[pos],
/* FP:lib.rs-2032 */                         raw_diffs[pos + 1],
/* FP:lib.rs-2033 */                         raw_diffs[pos + 2],
/* FP:lib.rs-2034 */                         raw_diffs[pos + 3],
/* FP:lib.rs-2035 */                     ];
/* FP:lib.rs-2036 */                     let diff = u32::from_le_bytes(bytes);
/* FP:lib.rs-2037 */                     line_start = line_start + RelativeBytePos(diff);
/* FP:lib.rs-2038 */                     line_start
/* FP:lib.rs-2039 */                 }));
/* FP:lib.rs-2040 */             }
/* FP:lib.rs-2041 */             _ => unreachable!(),
/* FP:lib.rs-2042 */         }
/* FP:lib.rs-2043 */ 
/* FP:lib.rs-2044 */         *guard = SourceFileLines::Lines(lines);
/* FP:lib.rs-2045 */ 
/* FP:lib.rs-2046 */         FreezeWriteGuard::freeze(guard);
/* FP:lib.rs-2047 */     }
/* FP:lib.rs-2048 */ 
/* FP:lib.rs-2049 */     pub fn lines(&self) -> &[RelativeBytePos] {
/* FP:lib.rs-2050 */         if let Some(SourceFileLines::Lines(lines)) = self.lines.get() {
/* FP:lib.rs-2051 */             return &lines[..];
/* FP:lib.rs-2052 */         }
/* FP:lib.rs-2053 */ 
/* FP:lib.rs-2054 */         outline(|| {
/* FP:lib.rs-2055 */             self.convert_diffs_to_lines_frozen();
/* FP:lib.rs-2056 */             if let Some(SourceFileLines::Lines(lines)) = self.lines.get() {
/* FP:lib.rs-2057 */                 return &lines[..];
/* FP:lib.rs-2058 */             }
/* FP:lib.rs-2059 */             unreachable!()
/* FP:lib.rs-2060 */         })
/* FP:lib.rs-2061 */     }
/* FP:lib.rs-2062 */ 
/* FP:lib.rs-2063 */     /// Returns the `BytePos` of the beginning of the current line.
/* FP:lib.rs-2064 */     pub fn line_begin_pos(&self, pos: BytePos) -> BytePos {
/* FP:lib.rs-2065 */         let pos = self.relative_position(pos);
/* FP:lib.rs-2066 */         let line_index = self.lookup_line(pos).unwrap();
/* FP:lib.rs-2067 */         let line_start_pos = self.lines()[line_index];
/* FP:lib.rs-2068 */         self.absolute_position(line_start_pos)
/* FP:lib.rs-2069 */     }
/* FP:lib.rs-2070 */ 
/* FP:lib.rs-2071 */     /// Add externally loaded source.
/* FP:lib.rs-2072 */     /// If the hash of the input doesn't match or no input is supplied via None,
/* FP:lib.rs-2073 */     /// it is interpreted as an error and the corresponding enum variant is set.
/* FP:lib.rs-2074 */     /// The return value signifies whether some kind of source is present.
/* FP:lib.rs-2075 */     pub fn add_external_src<F>(&self, get_src: F) -> bool
/* FP:lib.rs-2076 */     where
/* FP:lib.rs-2077 */         F: FnOnce() -> Option<String>,
/* FP:lib.rs-2078 */     {
/* FP:lib.rs-2079 */         if !self.external_src.is_frozen() {
/* FP:lib.rs-2080 */             let src = get_src();
/* FP:lib.rs-2081 */             let src = src.and_then(|mut src| {
/* FP:lib.rs-2082 */                 // The src_hash needs to be computed on the pre-normalized src.
/* FP:lib.rs-2083 */                 self.src_hash.matches(&src).then(|| {
/* FP:lib.rs-2084 */                     normalize_src(&mut src);
/* FP:lib.rs-2085 */                     src
/* FP:lib.rs-2086 */                 })
/* FP:lib.rs-2087 */             });
/* FP:lib.rs-2088 */ 
/* FP:lib.rs-2089 */             self.external_src.try_write().map(|mut external_src| {
/* FP:lib.rs-2090 */                 if let ExternalSource::Foreign {
/* FP:lib.rs-2091 */                     kind: src_kind @ ExternalSourceKind::AbsentOk,
/* FP:lib.rs-2092 */                     ..
/* FP:lib.rs-2093 */                 } = &mut *external_src
/* FP:lib.rs-2094 */                 {
/* FP:lib.rs-2095 */                     *src_kind = if let Some(src) = src {
/* FP:lib.rs-2096 */                         ExternalSourceKind::Present(Arc::new(src))
/* FP:lib.rs-2097 */                     } else {
/* FP:lib.rs-2098 */                         ExternalSourceKind::AbsentErr
/* FP:lib.rs-2099 */                     };
/* FP:lib.rs-2100 */                 } else {
/* FP:lib.rs-2101 */                     panic!("unexpected state {:?}", *external_src)
/* FP:lib.rs-2102 */                 }
/* FP:lib.rs-2103 */ 
/* FP:lib.rs-2104 */                 // Freeze this so we don't try to load the source again.
/* FP:lib.rs-2105 */                 FreezeWriteGuard::freeze(external_src)
/* FP:lib.rs-2106 */             });
/* FP:lib.rs-2107 */         }
/* FP:lib.rs-2108 */ 
/* FP:lib.rs-2109 */         self.src.is_some() || self.external_src.read().get_source().is_some()
/* FP:lib.rs-2110 */     }
/* FP:lib.rs-2111 */ 
/* FP:lib.rs-2112 */     /// Gets a line from the list of pre-computed line-beginnings.
/* FP:lib.rs-2113 */     /// The line number here is 0-based.
/* FP:lib.rs-2114 */     pub fn get_line(&self, line_number: usize) -> Option<Cow<'_, str>> {
/* FP:lib.rs-2115 */         fn get_until_newline(src: &str, begin: usize) -> &str {
/* FP:lib.rs-2116 */             // We can't use `lines.get(line_number+1)` because we might
/* FP:lib.rs-2117 */             // be parsing when we call this function and thus the current
/* FP:lib.rs-2118 */             // line is the last one we have line info for.
/* FP:lib.rs-2119 */             let slice = &src[begin..];
/* FP:lib.rs-2120 */             match slice.find('\n') {
/* FP:lib.rs-2121 */                 Some(e) => &slice[..e],
/* FP:lib.rs-2122 */                 None => slice,
/* FP:lib.rs-2123 */             }
/* FP:lib.rs-2124 */         }
/* FP:lib.rs-2125 */ 
/* FP:lib.rs-2126 */         let begin = {
/* FP:lib.rs-2127 */             let line = self.lines().get(line_number).copied()?;
/* FP:lib.rs-2128 */             line.to_usize()
/* FP:lib.rs-2129 */         };
/* FP:lib.rs-2130 */ 
/* FP:lib.rs-2131 */         if let Some(ref src) = self.src {
/* FP:lib.rs-2132 */             Some(Cow::from(get_until_newline(src, begin)))
/* FP:lib.rs-2133 */         } else {
/* FP:lib.rs-2134 */             self.external_src
/* FP:lib.rs-2135 */                 .borrow()
/* FP:lib.rs-2136 */                 .get_source()
/* FP:lib.rs-2137 */                 .map(|src| Cow::Owned(String::from(get_until_newline(src, begin))))
/* FP:lib.rs-2138 */         }
/* FP:lib.rs-2139 */     }
/* FP:lib.rs-2140 */ 
/* FP:lib.rs-2141 */     pub fn is_real_file(&self) -> bool {
/* FP:lib.rs-2142 */         self.name.is_real()
/* FP:lib.rs-2143 */     }
/* FP:lib.rs-2144 */ 
/* FP:lib.rs-2145 */     #[inline]
/* FP:lib.rs-2146 */     pub fn is_imported(&self) -> bool {
/* FP:lib.rs-2147 */         self.src.is_none()
/* FP:lib.rs-2148 */     }
/* FP:lib.rs-2149 */ 
/* FP:lib.rs-2150 */     pub fn count_lines(&self) -> usize {
/* FP:lib.rs-2151 */         self.lines().len()
/* FP:lib.rs-2152 */     }
/* FP:lib.rs-2153 */ 
/* FP:lib.rs-2154 */     #[inline]
/* FP:lib.rs-2155 */     pub fn absolute_position(&self, pos: RelativeBytePos) -> BytePos {
/* FP:lib.rs-2156 */         BytePos::from_u32(pos.to_u32() + self.start_pos.to_u32())
/* FP:lib.rs-2157 */     }
/* FP:lib.rs-2158 */ 
/* FP:lib.rs-2159 */     #[inline]
/* FP:lib.rs-2160 */     pub fn relative_position(&self, pos: BytePos) -> RelativeBytePos {
/* FP:lib.rs-2161 */         RelativeBytePos::from_u32(pos.to_u32() - self.start_pos.to_u32())
/* FP:lib.rs-2162 */     }
/* FP:lib.rs-2163 */ 
/* FP:lib.rs-2164 */     #[inline]
/* FP:lib.rs-2165 */     pub fn end_position(&self) -> BytePos {
/* FP:lib.rs-2166 */         self.absolute_position(self.source_len)
/* FP:lib.rs-2167 */     }
/* FP:lib.rs-2168 */ 
/* FP:lib.rs-2169 */     /// Finds the line containing the given position. The return value is the
/* FP:lib.rs-2170 */     /// index into the `lines` array of this `SourceFile`, not the 1-based line
/* FP:lib.rs-2171 */     /// number. If the source_file is empty or the position is located before the
/* FP:lib.rs-2172 */     /// first line, `None` is returned.
/* FP:lib.rs-2173 */     pub fn lookup_line(&self, pos: RelativeBytePos) -> Option<usize> {
/* FP:lib.rs-2174 */         self.lines().partition_point(|x| x <= &pos).checked_sub(1)
/* FP:lib.rs-2175 */     }
/* FP:lib.rs-2176 */ 
/* FP:lib.rs-2177 */     pub fn line_bounds(&self, line_index: usize) -> Range<BytePos> {
/* FP:lib.rs-2178 */         if self.is_empty() {
/* FP:lib.rs-2179 */             return self.start_pos..self.start_pos;
/* FP:lib.rs-2180 */         }
/* FP:lib.rs-2181 */ 
/* FP:lib.rs-2182 */         let lines = self.lines();
/* FP:lib.rs-2183 */         assert!(line_index < lines.len());
/* FP:lib.rs-2184 */         if line_index == (lines.len() - 1) {
/* FP:lib.rs-2185 */             self.absolute_position(lines[line_index])..self.end_position()
/* FP:lib.rs-2186 */         } else {
/* FP:lib.rs-2187 */             self.absolute_position(lines[line_index])..self.absolute_position(lines[line_index + 1])
/* FP:lib.rs-2188 */         }
/* FP:lib.rs-2189 */     }
/* FP:lib.rs-2190 */ 
/* FP:lib.rs-2191 */     /// Returns whether or not the file contains the given `SourceMap` byte
/* FP:lib.rs-2192 */     /// position. The position one past the end of the file is considered to be
/* FP:lib.rs-2193 */     /// contained by the file. This implies that files for which `is_empty`
/* FP:lib.rs-2194 */     /// returns true still contain one byte position according to this function.
/* FP:lib.rs-2195 */     #[inline]
/* FP:lib.rs-2196 */     pub fn contains(&self, byte_pos: BytePos) -> bool {
/* FP:lib.rs-2197 */         byte_pos >= self.start_pos && byte_pos <= self.end_position()
/* FP:lib.rs-2198 */     }
/* FP:lib.rs-2199 */ 
/* FP:lib.rs-2200 */     #[inline]
/* FP:lib.rs-2201 */     pub fn is_empty(&self) -> bool {
/* FP:lib.rs-2202 */         self.source_len.to_u32() == 0
/* FP:lib.rs-2203 */     }
/* FP:lib.rs-2204 */ 
/* FP:lib.rs-2205 */     /// Calculates the original byte position relative to the start of the file
/* FP:lib.rs-2206 */     /// based on the given byte position.
/* FP:lib.rs-2207 */     pub fn original_relative_byte_pos(&self, pos: BytePos) -> RelativeBytePos {
/* FP:lib.rs-2208 */         let pos = self.relative_position(pos);
/* FP:lib.rs-2209 */ 
/* FP:lib.rs-2210 */         // Diff before any records is 0. Otherwise use the previously recorded
/* FP:lib.rs-2211 */         // diff as that applies to the following characters until a new diff
/* FP:lib.rs-2212 */         // is recorded.
/* FP:lib.rs-2213 */         let diff = match self.normalized_pos.binary_search_by(|np| np.pos.cmp(&pos)) {
/* FP:lib.rs-2214 */             Ok(i) => self.normalized_pos[i].diff,
/* FP:lib.rs-2215 */             Err(0) => 0,
/* FP:lib.rs-2216 */             Err(i) => self.normalized_pos[i - 1].diff,
/* FP:lib.rs-2217 */         };
/* FP:lib.rs-2218 */ 
/* FP:lib.rs-2219 */         RelativeBytePos::from_u32(pos.0 + diff)
/* FP:lib.rs-2220 */     }
/* FP:lib.rs-2221 */ 
/* FP:lib.rs-2222 */     /// Calculates a normalized byte position from a byte offset relative to the
/* FP:lib.rs-2223 */     /// start of the file.
/* FP:lib.rs-2224 */     ///
/* FP:lib.rs-2225 */     /// When we get an inline assembler error from LLVM during codegen, we
/* FP:lib.rs-2226 */     /// import the expanded assembly code as a new `SourceFile`, which can then
/* FP:lib.rs-2227 */     /// be used for error reporting with spans. However the byte offsets given
/* FP:lib.rs-2228 */     /// to us by LLVM are relative to the start of the original buffer, not the
/* FP:lib.rs-2229 */     /// normalized one. Hence we need to convert those offsets to the normalized
/* FP:lib.rs-2230 */     /// form when constructing spans.
/* FP:lib.rs-2231 */     pub fn normalized_byte_pos(&self, offset: u32) -> BytePos {
/* FP:lib.rs-2232 */         let diff = match self
/* FP:lib.rs-2233 */             .normalized_pos
/* FP:lib.rs-2234 */             .binary_search_by(|np| (np.pos.0 + np.diff).cmp(&(self.start_pos.0 + offset)))
/* FP:lib.rs-2235 */         {
/* FP:lib.rs-2236 */             Ok(i) => self.normalized_pos[i].diff,
/* FP:lib.rs-2237 */             Err(0) => 0,
/* FP:lib.rs-2238 */             Err(i) => self.normalized_pos[i - 1].diff,
/* FP:lib.rs-2239 */         };
/* FP:lib.rs-2240 */ 
/* FP:lib.rs-2241 */         BytePos::from_u32(self.start_pos.0 + offset - diff)
/* FP:lib.rs-2242 */     }
/* FP:lib.rs-2243 */ 
/* FP:lib.rs-2244 */     /// Converts an relative `RelativeBytePos` to a `CharPos` relative to the `SourceFile`.
/* FP:lib.rs-2245 */     fn bytepos_to_file_charpos(&self, bpos: RelativeBytePos) -> CharPos {
/* FP:lib.rs-2246 */         // The number of extra bytes due to multibyte chars in the `SourceFile`.
/* FP:lib.rs-2247 */         let mut total_extra_bytes = 0;
/* FP:lib.rs-2248 */ 
/* FP:lib.rs-2249 */         for mbc in self.multibyte_chars.iter() {
/* FP:lib.rs-2250 */             debug!("{}-byte char at {:?}", mbc.bytes, mbc.pos);
/* FP:lib.rs-2251 */             if mbc.pos < bpos {
/* FP:lib.rs-2252 */                 // Every character is at least one byte, so we only
/* FP:lib.rs-2253 */                 // count the actual extra bytes.
/* FP:lib.rs-2254 */                 total_extra_bytes += mbc.bytes as u32 - 1;
/* FP:lib.rs-2255 */                 // We should never see a byte position in the middle of a
/* FP:lib.rs-2256 */                 // character.
/* FP:lib.rs-2257 */                 assert!(bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32);
/* FP:lib.rs-2258 */             } else {
/* FP:lib.rs-2259 */                 break;
/* FP:lib.rs-2260 */             }
/* FP:lib.rs-2261 */         }
/* FP:lib.rs-2262 */ 
/* FP:lib.rs-2263 */         assert!(total_extra_bytes <= bpos.to_u32());
/* FP:lib.rs-2264 */         CharPos(bpos.to_usize() - total_extra_bytes as usize)
/* FP:lib.rs-2265 */     }
/* FP:lib.rs-2266 */ 
/* FP:lib.rs-2267 */     /// Looks up the file's (1-based) line number and (0-based `CharPos`) column offset, for a
/* FP:lib.rs-2268 */     /// given `RelativeBytePos`.
/* FP:lib.rs-2269 */     fn lookup_file_pos(&self, pos: RelativeBytePos) -> (usize, CharPos) {
/* FP:lib.rs-2270 */         let chpos = self.bytepos_to_file_charpos(pos);
/* FP:lib.rs-2271 */         match self.lookup_line(pos) {
/* FP:lib.rs-2272 */             Some(a) => {
/* FP:lib.rs-2273 */                 let line = a + 1; // Line numbers start at 1
/* FP:lib.rs-2274 */                 let linebpos = self.lines()[a];
/* FP:lib.rs-2275 */                 let linechpos = self.bytepos_to_file_charpos(linebpos);
/* FP:lib.rs-2276 */                 let col = chpos - linechpos;
/* FP:lib.rs-2277 */                 debug!("byte pos {:?} is on the line at byte pos {:?}", pos, linebpos);
/* FP:lib.rs-2278 */                 debug!("char pos {:?} is on the line at char pos {:?}", chpos, linechpos);
/* FP:lib.rs-2279 */                 debug!("byte is on line: {}", line);
/* FP:lib.rs-2280 */                 assert!(chpos >= linechpos);
/* FP:lib.rs-2281 */                 (line, col)
/* FP:lib.rs-2282 */             }
/* FP:lib.rs-2283 */             None => (0, chpos),
/* FP:lib.rs-2284 */         }
/* FP:lib.rs-2285 */     }
/* FP:lib.rs-2286 */ 
/* FP:lib.rs-2287 */     /// Looks up the file's (1-based) line number, (0-based `CharPos`) column offset, and (0-based)
/* FP:lib.rs-2288 */     /// column offset when displayed, for a given `BytePos`.
/* FP:lib.rs-2289 */     pub fn lookup_file_pos_with_col_display(&self, pos: BytePos) -> (usize, CharPos, usize) {
/* FP:lib.rs-2290 */         let pos = self.relative_position(pos);
/* FP:lib.rs-2291 */         let (line, col_or_chpos) = self.lookup_file_pos(pos);
/* FP:lib.rs-2292 */         if line > 0 {
/* FP:lib.rs-2293 */             let Some(code) = self.get_line(line - 1) else {
/* FP:lib.rs-2294 */                 // If we don't have the code available, it is ok as a fallback to return the bytepos
/* FP:lib.rs-2295 */                 // instead of the "display" column, which is only used to properly show underlines
/* FP:lib.rs-2296 */                 // in the terminal.
/* FP:lib.rs-2297 */                 // FIXME: we'll want better handling of this in the future for the sake of tools
/* FP:lib.rs-2298 */                 // that want to use the display col instead of byte offsets to modify Rust code, but
/* FP:lib.rs-2299 */                 // that is a problem for another day, the previous code was already incorrect for
/* FP:lib.rs-2300 */                 // both displaying *and* third party tools using the json output naïvely.
/* FP:lib.rs-2301 */                 tracing::info!("couldn't find line {line} {:?}", self.name);
/* FP:lib.rs-2302 */                 return (line, col_or_chpos, col_or_chpos.0);
/* FP:lib.rs-2303 */             };
/* FP:lib.rs-2304 */             let display_col = code.chars().take(col_or_chpos.0).map(|ch| char_width(ch)).sum();
/* FP:lib.rs-2305 */             (line, col_or_chpos, display_col)
/* FP:lib.rs-2306 */         } else {
/* FP:lib.rs-2307 */             // This is never meant to happen?
/* FP:lib.rs-2308 */             (0, col_or_chpos, col_or_chpos.0)
/* FP:lib.rs-2309 */         }
/* FP:lib.rs-2310 */     }
/* FP:lib.rs-2311 */ }
/* FP:lib.rs-2312 */ 
/* FP:lib.rs-2313 */ pub fn char_width(ch: char) -> usize {
/* FP:lib.rs-2314 */     // FIXME: `unicode_width` sometimes disagrees with terminals on how wide a `char` is. For now,
/* FP:lib.rs-2315 */     // just accept that sometimes the code line will be longer than desired.
/* FP:lib.rs-2316 */     match ch {
/* FP:lib.rs-2317 */         '\t' => 4,
/* FP:lib.rs-2318 */         // Keep the following list in sync with `crate::rustc_errors::emitter::OUTPUT_REPLACEMENTS`. These
/* FP:lib.rs-2319 */         // are control points that we replace before printing with a visible codepoint for the sake
/* FP:lib.rs-2320 */         // of being able to point at them with underlines.
/* FP:lib.rs-2321 */         '\u{0000}' | '\u{0001}' | '\u{0002}' | '\u{0003}' | '\u{0004}' | '\u{0005}'
/* FP:lib.rs-2322 */         | '\u{0006}' | '\u{0007}' | '\u{0008}' | '\u{000B}' | '\u{000C}' | '\u{000D}'
/* FP:lib.rs-2323 */         | '\u{000E}' | '\u{000F}' | '\u{0010}' | '\u{0011}' | '\u{0012}' | '\u{0013}'
/* FP:lib.rs-2324 */         | '\u{0014}' | '\u{0015}' | '\u{0016}' | '\u{0017}' | '\u{0018}' | '\u{0019}'
/* FP:lib.rs-2325 */         | '\u{001A}' | '\u{001B}' | '\u{001C}' | '\u{001D}' | '\u{001E}' | '\u{001F}'
/* FP:lib.rs-2326 */         | '\u{007F}' | '\u{202A}' | '\u{202B}' | '\u{202D}' | '\u{202E}' | '\u{2066}'
/* FP:lib.rs-2327 */         | '\u{2067}' | '\u{2068}' | '\u{202C}' | '\u{2069}' => 1,
/* FP:lib.rs-2328 */         _ => unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1),
/* FP:lib.rs-2329 */     }
/* FP:lib.rs-2330 */ }
/* FP:lib.rs-2331 */ 
/* FP:lib.rs-2332 */ pub fn str_width(s: &str) -> usize {
/* FP:lib.rs-2333 */     s.chars().map(char_width).sum()
/* FP:lib.rs-2334 */ }
/* FP:lib.rs-2335 */ 
/* FP:lib.rs-2336 */ /// Normalizes the source code and records the normalizations.
/* FP:lib.rs-2337 */ fn normalize_src(src: &mut String) -> Vec<NormalizedPos> {
/* FP:lib.rs-2338 */     let mut normalized_pos = vec![];
/* FP:lib.rs-2339 */     remove_bom(src, &mut normalized_pos);
/* FP:lib.rs-2340 */     normalize_newlines(src, &mut normalized_pos);
/* FP:lib.rs-2341 */     normalized_pos
/* FP:lib.rs-2342 */ }
/* FP:lib.rs-2343 */ 
/* FP:lib.rs-2344 */ /// Removes UTF-8 BOM, if any.
/* FP:lib.rs-2345 */ fn remove_bom(src: &mut String, normalized_pos: &mut Vec<NormalizedPos>) {
/* FP:lib.rs-2346 */     if src.starts_with('\u{feff}') {
/* FP:lib.rs-2347 */         src.drain(..3);
/* FP:lib.rs-2348 */         normalized_pos.push(NormalizedPos { pos: RelativeBytePos(0), diff: 3 });
/* FP:lib.rs-2349 */     }
/* FP:lib.rs-2350 */ }
/* FP:lib.rs-2351 */ 
/* FP:lib.rs-2352 */ /// Replaces `\r\n` with `\n` in-place in `src`.
/* FP:lib.rs-2353 */ ///
/* FP:lib.rs-2354 */ /// Leaves any occurrences of lone `\r` unchanged.
/* FP:lib.rs-2355 */ fn normalize_newlines(src: &mut String, normalized_pos: &mut Vec<NormalizedPos>) {
/* FP:lib.rs-2356 */     if !src.as_bytes().contains(&b'\r') {
/* FP:lib.rs-2357 */         return;
/* FP:lib.rs-2358 */     }
/* FP:lib.rs-2359 */ 
/* FP:lib.rs-2360 */     // We replace `\r\n` with `\n` in-place, which doesn't break utf-8 encoding.
/* FP:lib.rs-2361 */     // While we *can* call `as_mut_vec` and do surgery on the live string
/* FP:lib.rs-2362 */     // directly, let's rather steal the contents of `src`. This makes the code
/* FP:lib.rs-2363 */     // safe even if a panic occurs.
/* FP:lib.rs-2364 */ 
/* FP:lib.rs-2365 */     let mut buf = std::mem::replace(src, String::new()).into_bytes();
/* FP:lib.rs-2366 */     let mut gap_len = 0;
/* FP:lib.rs-2367 */     let mut tail = buf.as_mut_slice();
/* FP:lib.rs-2368 */     let mut cursor = 0;
/* FP:lib.rs-2369 */     let original_gap = normalized_pos.last().map_or(0, |l| l.diff);
/* FP:lib.rs-2370 */     loop {
/* FP:lib.rs-2371 */         let idx = match find_crlf(&tail[gap_len..]) {
/* FP:lib.rs-2372 */             None => tail.len(),
/* FP:lib.rs-2373 */             Some(idx) => idx + gap_len,
/* FP:lib.rs-2374 */         };
/* FP:lib.rs-2375 */         tail.copy_within(gap_len..idx, 0);
/* FP:lib.rs-2376 */         tail = &mut tail[idx - gap_len..];
/* FP:lib.rs-2377 */         if tail.len() == gap_len {
/* FP:lib.rs-2378 */             break;
/* FP:lib.rs-2379 */         }
/* FP:lib.rs-2380 */         cursor += idx - gap_len;
/* FP:lib.rs-2381 */         gap_len += 1;
/* FP:lib.rs-2382 */         normalized_pos.push(NormalizedPos {
/* FP:lib.rs-2383 */             pos: RelativeBytePos::from_usize(cursor + 1),
/* FP:lib.rs-2384 */             diff: original_gap + gap_len as u32,
/* FP:lib.rs-2385 */         });
/* FP:lib.rs-2386 */     }
/* FP:lib.rs-2387 */ 
/* FP:lib.rs-2388 */     // Account for removed `\r`.
/* FP:lib.rs-2389 */     // After `set_len`, `buf` is guaranteed to contain utf-8 again.
/* FP:lib.rs-2390 */     let new_len = buf.len() - gap_len;
/* FP:lib.rs-2391 */     unsafe {
/* FP:lib.rs-2392 */         buf.set_len(new_len);
/* FP:lib.rs-2393 */         *src = String::from_utf8_unchecked(buf);
/* FP:lib.rs-2394 */     }
/* FP:lib.rs-2395 */ 
/* FP:lib.rs-2396 */     fn find_crlf(src: &[u8]) -> Option<usize> {
/* FP:lib.rs-2397 */         let mut search_idx = 0;
/* FP:lib.rs-2398 */         while let Some(idx) = find_cr(&src[search_idx..]) {
/* FP:lib.rs-2399 */             if src[search_idx..].get(idx + 1) != Some(&b'\n') {
/* FP:lib.rs-2400 */                 search_idx += idx + 1;
/* FP:lib.rs-2401 */                 continue;
/* FP:lib.rs-2402 */             }
/* FP:lib.rs-2403 */             return Some(search_idx + idx);
/* FP:lib.rs-2404 */         }
/* FP:lib.rs-2405 */         None
/* FP:lib.rs-2406 */     }
/* FP:lib.rs-2407 */ 
/* FP:lib.rs-2408 */     fn find_cr(src: &[u8]) -> Option<usize> {
/* FP:lib.rs-2409 */         src.iter().position(|&b| b == b'\r')
/* FP:lib.rs-2410 */     }
/* FP:lib.rs-2411 */ }
/* FP:lib.rs-2412 */ 
/* FP:lib.rs-2413 */ // _____________________________________________________________________________
/* FP:lib.rs-2414 */ // Pos, BytePos, CharPos
/* FP:lib.rs-2415 */ //
/* FP:lib.rs-2416 */ 
/* FP:lib.rs-2417 */ pub trait Pos {
/* FP:lib.rs-2418 */     fn from_usize(n: usize) -> Self;
/* FP:lib.rs-2419 */     fn to_usize(&self) -> usize;
/* FP:lib.rs-2420 */     fn from_u32(n: u32) -> Self;
/* FP:lib.rs-2421 */     fn to_u32(&self) -> u32;
/* FP:lib.rs-2422 */ }
/* FP:lib.rs-2423 */ 
/* FP:lib.rs-2424 */ macro_rules! impl_pos {
/* FP:lib.rs-2425 */     (
/* FP:lib.rs-2426 */         $(
/* FP:lib.rs-2427 */             $(#[$attr:meta])*
/* FP:lib.rs-2428 */             $vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
/* FP:lib.rs-2429 */         )*
/* FP:lib.rs-2430 */     ) => {
/* FP:lib.rs-2431 */         $(
/* FP:lib.rs-2432 */             $(#[$attr])*
/* FP:lib.rs-2433 */             $vis struct $ident($inner_vis $inner_ty);
/* FP:lib.rs-2434 */ 
/* FP:lib.rs-2435 */             impl Pos for $ident {
/* FP:lib.rs-2436 */                 #[inline(always)]
/* FP:lib.rs-2437 */                 fn from_usize(n: usize) -> $ident {
/* FP:lib.rs-2438 */                     $ident(n as $inner_ty)
/* FP:lib.rs-2439 */                 }
/* FP:lib.rs-2440 */ 
/* FP:lib.rs-2441 */                 #[inline(always)]
/* FP:lib.rs-2442 */                 fn to_usize(&self) -> usize {
/* FP:lib.rs-2443 */                     self.0 as usize
/* FP:lib.rs-2444 */                 }
/* FP:lib.rs-2445 */ 
/* FP:lib.rs-2446 */                 #[inline(always)]
/* FP:lib.rs-2447 */                 fn from_u32(n: u32) -> $ident {
/* FP:lib.rs-2448 */                     $ident(n as $inner_ty)
/* FP:lib.rs-2449 */                 }
/* FP:lib.rs-2450 */ 
/* FP:lib.rs-2451 */                 #[inline(always)]
/* FP:lib.rs-2452 */                 fn to_u32(&self) -> u32 {
/* FP:lib.rs-2453 */                     self.0 as u32
/* FP:lib.rs-2454 */                 }
/* FP:lib.rs-2455 */             }
/* FP:lib.rs-2456 */ 
/* FP:lib.rs-2457 */             impl Add for $ident {
/* FP:lib.rs-2458 */                 type Output = $ident;
/* FP:lib.rs-2459 */ 
/* FP:lib.rs-2460 */                 #[inline(always)]
/* FP:lib.rs-2461 */                 fn add(self, rhs: $ident) -> $ident {
/* FP:lib.rs-2462 */                     $ident(self.0 + rhs.0)
/* FP:lib.rs-2463 */                 }
/* FP:lib.rs-2464 */             }
/* FP:lib.rs-2465 */ 
/* FP:lib.rs-2466 */             impl Sub for $ident {
/* FP:lib.rs-2467 */                 type Output = $ident;
/* FP:lib.rs-2468 */ 
/* FP:lib.rs-2469 */                 #[inline(always)]
/* FP:lib.rs-2470 */                 fn sub(self, rhs: $ident) -> $ident {
/* FP:lib.rs-2471 */                     $ident(self.0 - rhs.0)
/* FP:lib.rs-2472 */                 }
/* FP:lib.rs-2473 */             }
/* FP:lib.rs-2474 */         )*
/* FP:lib.rs-2475 */     };
/* FP:lib.rs-2476 */ }
/* FP:lib.rs-2477 */ 
/* FP:lib.rs-2478 */ impl_pos! {
/* FP:lib.rs-2479 */     /// A byte offset.
/* FP:lib.rs-2480 */     ///
/* FP:lib.rs-2481 */     /// Keep this small (currently 32-bits), as AST contains a lot of them.
/* FP:lib.rs-2482 */     #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
/* FP:lib.rs-2483 */     pub struct BytePos(pub u32);
/* FP:lib.rs-2484 */ 
/* FP:lib.rs-2485 */     /// A byte offset relative to file beginning.
/* FP:lib.rs-2486 */     #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
/* FP:lib.rs-2487 */     pub struct RelativeBytePos(pub u32);
/* FP:lib.rs-2488 */ 
/* FP:lib.rs-2489 */     /// A character offset.
/* FP:lib.rs-2490 */     ///
/* FP:lib.rs-2491 */     /// Because of multibyte UTF-8 characters, a byte offset
/* FP:lib.rs-2492 */     /// is not equivalent to a character offset. The [`SourceMap`] will convert [`BytePos`]
/* FP:lib.rs-2493 */     /// values to `CharPos` values as necessary.
/* FP:lib.rs-2494 */     #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
/* FP:lib.rs-2495 */     pub struct CharPos(pub usize);
/* FP:lib.rs-2496 */ }
/* FP:lib.rs-2497 */ 
/* FP:lib.rs-2498 */ impl<S: Encoder> Encodable<S> for BytePos {
/* FP:lib.rs-2499 */     fn encode(&self, s: &mut S) {
/* FP:lib.rs-2500 */         s.emit_u32(self.0);
/* FP:lib.rs-2501 */     }
/* FP:lib.rs-2502 */ }
/* FP:lib.rs-2503 */ 
/* FP:lib.rs-2504 */ impl<D: Decoder> Decodable<D> for BytePos {
/* FP:lib.rs-2505 */     fn decode(d: &mut D) -> BytePos {
/* FP:lib.rs-2506 */         BytePos(d.read_u32())
/* FP:lib.rs-2507 */     }
/* FP:lib.rs-2508 */ }
/* FP:lib.rs-2509 */ 
/* FP:lib.rs-2510 */ impl<H: HashStableContext> HashStable<H> for RelativeBytePos {
/* FP:lib.rs-2511 */     fn hash_stable(&self, hcx: &mut H, hasher: &mut StableHasher) {
/* FP:lib.rs-2512 */         self.0.hash_stable(hcx, hasher);
/* FP:lib.rs-2513 */     }
/* FP:lib.rs-2514 */ }
/* FP:lib.rs-2515 */ 
/* FP:lib.rs-2516 */ impl<S: Encoder> Encodable<S> for RelativeBytePos {
/* FP:lib.rs-2517 */     fn encode(&self, s: &mut S) {
/* FP:lib.rs-2518 */         s.emit_u32(self.0);
/* FP:lib.rs-2519 */     }
/* FP:lib.rs-2520 */ }
/* FP:lib.rs-2521 */ 
/* FP:lib.rs-2522 */ impl<D: Decoder> Decodable<D> for RelativeBytePos {
/* FP:lib.rs-2523 */     fn decode(d: &mut D) -> RelativeBytePos {
/* FP:lib.rs-2524 */         RelativeBytePos(d.read_u32())
/* FP:lib.rs-2525 */     }
/* FP:lib.rs-2526 */ }
/* FP:lib.rs-2527 */ 
/* FP:lib.rs-2528 */ // _____________________________________________________________________________
/* FP:lib.rs-2529 */ // Loc, SourceFileAndLine, SourceFileAndBytePos
/* FP:lib.rs-2530 */ //
/* FP:lib.rs-2531 */ 
/* FP:lib.rs-2532 */ /// A source code location used for error reporting.
/* FP:lib.rs-2533 */ #[derive(Debug, Clone)]
/* FP:lib.rs-2534 */ pub struct Loc {
/* FP:lib.rs-2535 */     /// Information about the original source.
/* FP:lib.rs-2536 */     pub file: Arc<SourceFile>,
/* FP:lib.rs-2537 */     /// The (1-based) line number.
/* FP:lib.rs-2538 */     pub line: usize,
/* FP:lib.rs-2539 */     /// The (0-based) column offset.
/* FP:lib.rs-2540 */     pub col: CharPos,
/* FP:lib.rs-2541 */     /// The (0-based) column offset when displayed.
/* FP:lib.rs-2542 */     pub col_display: usize,
/* FP:lib.rs-2543 */ }
/* FP:lib.rs-2544 */ 
/* FP:lib.rs-2545 */ // Used to be structural records.
/* FP:lib.rs-2546 */ #[derive(Debug)]
/* FP:lib.rs-2547 */ pub struct SourceFileAndLine {
/* FP:lib.rs-2548 */     pub sf: Arc<SourceFile>,
/* FP:lib.rs-2549 */     /// Index of line, starting from 0.
/* FP:lib.rs-2550 */     pub line: usize,
/* FP:lib.rs-2551 */ }
/* FP:lib.rs-2552 */ #[derive(Debug)]
/* FP:lib.rs-2553 */ pub struct SourceFileAndBytePos {
/* FP:lib.rs-2554 */     pub sf: Arc<SourceFile>,
/* FP:lib.rs-2555 */     pub pos: BytePos,
/* FP:lib.rs-2556 */ }
/* FP:lib.rs-2557 */ 
/* FP:lib.rs-2558 */ #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/* FP:lib.rs-2559 */ pub struct LineInfo {
/* FP:lib.rs-2560 */     /// Index of line, starting from 0.
/* FP:lib.rs-2561 */     pub line_index: usize,
/* FP:lib.rs-2562 */ 
/* FP:lib.rs-2563 */     /// Column in line where span begins, starting from 0.
/* FP:lib.rs-2564 */     pub start_col: CharPos,
/* FP:lib.rs-2565 */ 
/* FP:lib.rs-2566 */     /// Column in line where span ends, starting from 0, exclusive.
/* FP:lib.rs-2567 */     pub end_col: CharPos,
/* FP:lib.rs-2568 */ }
/* FP:lib.rs-2569 */ 
/* FP:lib.rs-2570 */ pub struct FileLines {
/* FP:lib.rs-2571 */     pub file: Arc<SourceFile>,
/* FP:lib.rs-2572 */     pub lines: Vec<LineInfo>,
/* FP:lib.rs-2573 */ }
/* FP:lib.rs-2574 */ 
/* FP:lib.rs-2575 */ pub static SPAN_TRACK: AtomicRef<fn(LocalDefId)> = AtomicRef::new(&((|_| {}) as fn(_)));
/* FP:lib.rs-2576 */ 
/* FP:lib.rs-2577 */ // _____________________________________________________________________________
/* FP:lib.rs-2578 */ // SpanLinesError, SpanSnippetError, DistinctSources, MalformedSourceMapPositions
/* FP:lib.rs-2579 */ //
/* FP:lib.rs-2580 */ 
/* FP:lib.rs-2581 */ pub type FileLinesResult = Result<FileLines, SpanLinesError>;
/* FP:lib.rs-2582 */ 
/* FP:lib.rs-2583 */ #[derive(Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-2584 */ pub enum SpanLinesError {
/* FP:lib.rs-2585 */     DistinctSources(Box<DistinctSources>),
/* FP:lib.rs-2586 */ }
/* FP:lib.rs-2587 */ 
/* FP:lib.rs-2588 */ #[derive(Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-2589 */ pub enum SpanSnippetError {
/* FP:lib.rs-2590 */     IllFormedSpan(Span),
/* FP:lib.rs-2591 */     DistinctSources(Box<DistinctSources>),
/* FP:lib.rs-2592 */     MalformedForSourcemap(MalformedSourceMapPositions),
/* FP:lib.rs-2593 */     SourceNotAvailable { filename: FileName },
/* FP:lib.rs-2594 */ }
/* FP:lib.rs-2595 */ 
/* FP:lib.rs-2596 */ #[derive(Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-2597 */ pub struct DistinctSources {
/* FP:lib.rs-2598 */     pub begin: (FileName, BytePos),
/* FP:lib.rs-2599 */     pub end: (FileName, BytePos),
/* FP:lib.rs-2600 */ }
/* FP:lib.rs-2601 */ 
/* FP:lib.rs-2602 */ #[derive(Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-2603 */ pub struct MalformedSourceMapPositions {
/* FP:lib.rs-2604 */     pub name: FileName,
/* FP:lib.rs-2605 */     pub source_len: usize,
/* FP:lib.rs-2606 */     pub begin_pos: BytePos,
/* FP:lib.rs-2607 */     pub end_pos: BytePos,
/* FP:lib.rs-2608 */ }
/* FP:lib.rs-2609 */ 
/* FP:lib.rs-2610 */ /// Range inside of a `Span` used for diagnostics when we only have access to relative positions.
/* FP:lib.rs-2611 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-2612 */ pub struct InnerSpan {
/* FP:lib.rs-2613 */     pub start: usize,
/* FP:lib.rs-2614 */     pub end: usize,
/* FP:lib.rs-2615 */ }
/* FP:lib.rs-2616 */ 
/* FP:lib.rs-2617 */ impl InnerSpan {
/* FP:lib.rs-2618 */     pub fn new(start: usize, end: usize) -> InnerSpan {
/* FP:lib.rs-2619 */         InnerSpan { start, end }
/* FP:lib.rs-2620 */     }
/* FP:lib.rs-2621 */ }
/* FP:lib.rs-2622 */ 
/* FP:lib.rs-2623 */ /// Requirements for a `StableHashingContext` to be used in this crate.
/* FP:lib.rs-2624 */ ///
/* FP:lib.rs-2625 */ /// This is a hack to allow using the [`HashStable_Generic`] derive macro
/* FP:lib.rs-2626 */ /// instead of implementing everything in rustc_middle.
/* FP:lib.rs-2627 */ pub trait HashStableContext {
/* FP:lib.rs-2628 */     fn def_path_hash(&self, def_id: DefId) -> DefPathHash;
/* FP:lib.rs-2629 */     fn hash_spans(&self) -> bool;
/* FP:lib.rs-2630 */     /// Accesses `sess.opts.unstable_opts.incremental_ignore_spans` since
/* FP:lib.rs-2631 */     /// we don't have easy access to a `Session`
/* FP:lib.rs-2632 */     fn unstable_opts_incremental_ignore_spans(&self) -> bool;
/* FP:lib.rs-2633 */     fn def_span(&self, def_id: LocalDefId) -> Span;
/* FP:lib.rs-2634 */     fn span_data_to_lines_and_cols(
/* FP:lib.rs-2635 */         &mut self,
/* FP:lib.rs-2636 */         span: &SpanData,
/* FP:lib.rs-2637 */     ) -> Option<(StableSourceFileId, usize, BytePos, usize, BytePos)>;
/* FP:lib.rs-2638 */     fn hashing_controls(&self) -> HashingControls;
/* FP:lib.rs-2639 */ }
/* FP:lib.rs-2640 */ 
/* FP:lib.rs-2641 */ impl<CTX> HashStable<CTX> for Span
/* FP:lib.rs-2642 */ where
/* FP:lib.rs-2643 */     CTX: HashStableContext,
/* FP:lib.rs-2644 */ {
/* FP:lib.rs-2645 */     /// Hashes a span in a stable way. We can't directly hash the span's `BytePos`
/* FP:lib.rs-2646 */     /// fields (that would be similar to hashing pointers, since those are just
/* FP:lib.rs-2647 */     /// offsets into the `SourceMap`). Instead, we hash the (file name, line, column)
/* FP:lib.rs-2648 */     /// triple, which stays the same even if the containing `SourceFile` has moved
/* FP:lib.rs-2649 */     /// within the `SourceMap`.
/* FP:lib.rs-2650 */     ///
/* FP:lib.rs-2651 */     /// Also note that we are hashing byte offsets for the column, not unicode
/* FP:lib.rs-2652 */     /// codepoint offsets. For the purpose of the hash that's sufficient.
/* FP:lib.rs-2653 */     /// Also, hashing filenames is expensive so we avoid doing it twice when the
/* FP:lib.rs-2654 */     /// span starts and ends in the same file, which is almost always the case.
/* FP:lib.rs-2655 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:lib.rs-2656 */         const TAG_VALID_SPAN: u8 = 0;
/* FP:lib.rs-2657 */         const TAG_INVALID_SPAN: u8 = 1;
/* FP:lib.rs-2658 */         const TAG_RELATIVE_SPAN: u8 = 2;
/* FP:lib.rs-2659 */ 
/* FP:lib.rs-2660 */         if !ctx.hash_spans() {
/* FP:lib.rs-2661 */             return;
/* FP:lib.rs-2662 */         }
/* FP:lib.rs-2663 */ 
/* FP:lib.rs-2664 */         let span = self.data_untracked();
/* FP:lib.rs-2665 */         span.ctxt.hash_stable(ctx, hasher);
/* FP:lib.rs-2666 */         span.parent.hash_stable(ctx, hasher);
/* FP:lib.rs-2667 */ 
/* FP:lib.rs-2668 */         if span.is_dummy() {
/* FP:lib.rs-2669 */             Hash::hash(&TAG_INVALID_SPAN, hasher);
/* FP:lib.rs-2670 */             return;
/* FP:lib.rs-2671 */         }
/* FP:lib.rs-2672 */ 
/* FP:lib.rs-2673 */         if let Some(parent) = span.parent {
/* FP:lib.rs-2674 */             let def_span = ctx.def_span(parent).data_untracked();
/* FP:lib.rs-2675 */             if def_span.contains(span) {
/* FP:lib.rs-2676 */                 // This span is enclosed in a definition: only hash the relative position.
/* FP:lib.rs-2677 */                 Hash::hash(&TAG_RELATIVE_SPAN, hasher);
/* FP:lib.rs-2678 */                 (span.lo - def_span.lo).to_u32().hash_stable(ctx, hasher);
/* FP:lib.rs-2679 */                 (span.hi - def_span.lo).to_u32().hash_stable(ctx, hasher);
/* FP:lib.rs-2680 */                 return;
/* FP:lib.rs-2681 */             }
/* FP:lib.rs-2682 */         }
/* FP:lib.rs-2683 */ 
/* FP:lib.rs-2684 */         // If this is not an empty or invalid span, we want to hash the last
/* FP:lib.rs-2685 */         // position that belongs to it, as opposed to hashing the first
/* FP:lib.rs-2686 */         // position past it.
/* FP:lib.rs-2687 */         let Some((file, line_lo, col_lo, line_hi, col_hi)) = ctx.span_data_to_lines_and_cols(&span)
/* FP:lib.rs-2688 */         else {
/* FP:lib.rs-2689 */             Hash::hash(&TAG_INVALID_SPAN, hasher);
/* FP:lib.rs-2690 */             return;
/* FP:lib.rs-2691 */         };
/* FP:lib.rs-2692 */ 
/* FP:lib.rs-2693 */         Hash::hash(&TAG_VALID_SPAN, hasher);
/* FP:lib.rs-2694 */         Hash::hash(&file, hasher);
/* FP:lib.rs-2695 */ 
/* FP:lib.rs-2696 */         // Hash both the length and the end location (line/column) of a span. If we
/* FP:lib.rs-2697 */         // hash only the length, for example, then two otherwise equal spans with
/* FP:lib.rs-2698 */         // different end locations will have the same hash. This can cause a problem
/* FP:lib.rs-2699 */         // during incremental compilation wherein a previous result for a query that
/* FP:lib.rs-2700 */         // depends on the end location of a span will be incorrectly reused when the
/* FP:lib.rs-2701 */         // end location of the span it depends on has changed (see issue #74890). A
/* FP:lib.rs-2702 */         // similar analysis applies if some query depends specifically on the length
/* FP:lib.rs-2703 */         // of the span, but we only hash the end location. So hash both.
/* FP:lib.rs-2704 */ 
/* FP:lib.rs-2705 */         let col_lo_trunc = (col_lo.0 as u64) & 0xFF;
/* FP:lib.rs-2706 */         let line_lo_trunc = ((line_lo as u64) & 0xFF_FF_FF) << 8;
/* FP:lib.rs-2707 */         let col_hi_trunc = (col_hi.0 as u64) & 0xFF << 32;
/* FP:lib.rs-2708 */         let line_hi_trunc = ((line_hi as u64) & 0xFF_FF_FF) << 40;
/* FP:lib.rs-2709 */         let col_line = col_lo_trunc | line_lo_trunc | col_hi_trunc | line_hi_trunc;
/* FP:lib.rs-2710 */         let len = (span.hi - span.lo).0;
/* FP:lib.rs-2711 */         Hash::hash(&col_line, hasher);
/* FP:lib.rs-2712 */         Hash::hash(&len, hasher);
/* FP:lib.rs-2713 */     }
/* FP:lib.rs-2714 */ }
/* FP:lib.rs-2715 */ 
/* FP:lib.rs-2716 */ /// Useful type to use with `Result<>` indicate that an error has already
/* FP:lib.rs-2717 */ /// been reported to the user, so no need to continue checking.
/* FP:lib.rs-2718 */ ///
/* FP:lib.rs-2719 */ /// The `()` field is necessary: it is non-`pub`, which means values of this
/* FP:lib.rs-2720 */ /// type cannot be constructed outside of this crate.
/* FP:lib.rs-2721 */ #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
/* FP:lib.rs-2722 */ #[derive(HashStable_Generic)]
/* FP:lib.rs-2723 */ pub struct ErrorGuaranteed(());
/* FP:lib.rs-2724 */ 
/* FP:lib.rs-2725 */ impl ErrorGuaranteed {
/* FP:lib.rs-2726 */     /// Don't use this outside of `DiagCtxtInner::emit_diagnostic`!
/* FP:lib.rs-2727 */     #[deprecated = "should only be used in `DiagCtxtInner::emit_diagnostic`"]
/* FP:lib.rs-2728 */     pub fn unchecked_error_guaranteed() -> Self {
/* FP:lib.rs-2729 */         ErrorGuaranteed(())
/* FP:lib.rs-2730 */     }
/* FP:lib.rs-2731 */ 
/* FP:lib.rs-2732 */     pub fn raise_fatal(self) -> ! {
/* FP:lib.rs-2733 */         FatalError.raise()
/* FP:lib.rs-2734 */     }
/* FP:lib.rs-2735 */ }
/* FP:lib.rs-2736 */ 
/* FP:lib.rs-2737 */ impl<E: crate::rustc_serialize::Encoder> Encodable<E> for ErrorGuaranteed {
/* FP:lib.rs-2738 */     #[inline]
/* FP:lib.rs-2739 */     fn encode(&self, _e: &mut E) {
/* FP:lib.rs-2740 */         panic!(
/* FP:lib.rs-2741 */             "should never serialize an `ErrorGuaranteed`, as we do not write metadata or \
/* FP:lib.rs-2742 */             incremental caches in case errors occurred"
/* FP:lib.rs-2743 */         )
/* FP:lib.rs-2744 */     }
/* FP:lib.rs-2745 */ }
/* FP:lib.rs-2746 */ impl<D: crate::rustc_serialize::Decoder> Decodable<D> for ErrorGuaranteed {
/* FP:lib.rs-2747 */     #[inline]
/* FP:lib.rs-2748 */     fn decode(_d: &mut D) -> ErrorGuaranteed {
/* FP:lib.rs-2749 */         panic!(
/* FP:lib.rs-2750 */             "`ErrorGuaranteed` should never have been serialized to metadata or incremental caches"
/* FP:lib.rs-2751 */         )
/* FP:lib.rs-2752 */     }
/* FP:lib.rs-2753 */ }