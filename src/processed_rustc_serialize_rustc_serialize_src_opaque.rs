/* FP:opaque.rs-0001 */ use std::fs::File;
/* FP:opaque.rs-0002 */ use std::io::{self, Write};
/* FP:opaque.rs-0003 */ use std::marker::PhantomData;
/* FP:opaque.rs-0004 */ use std::ops::Range;
/* FP:opaque.rs-0005 */ use std::path::{Path, PathBuf};
/* FP:opaque.rs-0006 */ 
/* FP:opaque.rs-0007 */ // This code is very hot and uses lots of arithmetic, avoid overflow checks for performance.
/* FP:opaque.rs-0008 */ // See https://github.com/rust-lang/rust/pull/119440#issuecomment-1874255727
/* FP:opaque.rs-0009 */ use crate::int_overflow::DebugStrictAdd;
/* FP:opaque.rs-0010 */ use crate::leb128;
/* FP:opaque.rs-0011 */ use crate::serialize::{Decodable, Decoder, Encodable, Encoder};
/* FP:opaque.rs-0012 */ 
/* FP:opaque.rs-0014 */ 
/* FP:opaque.rs-0015 */ // -----------------------------------------------------------------------------
/* FP:opaque.rs-0016 */ // Encoder
/* FP:opaque.rs-0017 */ // -----------------------------------------------------------------------------
/* FP:opaque.rs-0018 */ 
/* FP:opaque.rs-0019 */ pub type FileEncodeResult = Result<usize, (PathBuf, io::Error)>;
/* FP:opaque.rs-0020 */ 
/* FP:opaque.rs-0021 */ pub const MAGIC_END_BYTES: &[u8] = b"rust-end-file";
/* FP:opaque.rs-0022 */ 
/* FP:opaque.rs-0023 */ /// The size of the buffer in `FileEncoder`.
/* FP:opaque.rs-0024 */ const BUF_SIZE: usize = 64 * 1024;
/* FP:opaque.rs-0025 */ 
/* FP:opaque.rs-0026 */ /// `FileEncoder` encodes data to file via fixed-size buffer.
/* FP:opaque.rs-0027 */ ///
/* FP:opaque.rs-0028 */ /// There used to be a `MemEncoder` type that encoded all the data into a
/* FP:opaque.rs-0029 */ /// `Vec`. `FileEncoder` is better because its memory use is determined by the
/* FP:opaque.rs-0030 */ /// size of the buffer, rather than the full length of the encoded data, and
/* FP:opaque.rs-0031 */ /// because it doesn't need to reallocate memory along the way.
/* FP:opaque.rs-0032 */ pub struct FileEncoder {
/* FP:opaque.rs-0033 */     // The input buffer. For adequate performance, we need to be able to write
/* FP:opaque.rs-0034 */     // directly to the unwritten region of the buffer, without calling copy_from_slice.
/* FP:opaque.rs-0035 */     // Note that our buffer is always initialized so that we can do that direct access
/* FP:opaque.rs-0036 */     // without unsafe code. Users of this type write many more than BUF_SIZE bytes, so the
/* FP:opaque.rs-0037 */     // initialization is approximately free.
/* FP:opaque.rs-0038 */     buf: Box<[u8; BUF_SIZE]>,
/* FP:opaque.rs-0039 */     buffered: usize,
/* FP:opaque.rs-0040 */     flushed: usize,
/* FP:opaque.rs-0041 */     file: File,
/* FP:opaque.rs-0042 */     // This is used to implement delayed error handling, as described in the
/* FP:opaque.rs-0043 */     // comment on `trait Encoder`.
/* FP:opaque.rs-0044 */     res: Result<(), io::Error>,
/* FP:opaque.rs-0045 */     path: PathBuf,
/* FP:opaque.rs-0046 */     #[cfg(debug_assertions)]
/* FP:opaque.rs-0047 */     finished: bool,
/* FP:opaque.rs-0048 */ }
/* FP:opaque.rs-0049 */ 
/* FP:opaque.rs-0050 */ impl FileEncoder {
/* FP:opaque.rs-0051 */     pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
/* FP:opaque.rs-0052 */         // File::create opens the file for writing only. When -Zmeta-stats is enabled, the metadata
/* FP:opaque.rs-0053 */         // encoder rewinds the file to inspect what was written. So we need to always open the file
/* FP:opaque.rs-0054 */         // for reading and writing.
/* FP:opaque.rs-0055 */         let file =
/* FP:opaque.rs-0056 */             File::options().read(true).write(true).create(true).truncate(true).open(&path)?;
/* FP:opaque.rs-0057 */ 
/* FP:opaque.rs-0058 */         Ok(FileEncoder {
/* FP:opaque.rs-0059 */             buf: vec![0u8; BUF_SIZE].into_boxed_slice().try_into().unwrap(),
/* FP:opaque.rs-0060 */             path: path.as_ref().into(),
/* FP:opaque.rs-0061 */             buffered: 0,
/* FP:opaque.rs-0062 */             flushed: 0,
/* FP:opaque.rs-0063 */             file,
/* FP:opaque.rs-0064 */             res: Ok(()),
/* FP:opaque.rs-0065 */             #[cfg(debug_assertions)]
/* FP:opaque.rs-0066 */             finished: false,
/* FP:opaque.rs-0067 */         })
/* FP:opaque.rs-0068 */     }
/* FP:opaque.rs-0069 */ 
/* FP:opaque.rs-0070 */     #[inline]
/* FP:opaque.rs-0071 */     pub fn position(&self) -> usize {
/* FP:opaque.rs-0072 */         // Tracking position this way instead of having a `self.position` field
/* FP:opaque.rs-0073 */         // means that we only need to update `self.buffered` on a write call,
/* FP:opaque.rs-0074 */         // as opposed to updating `self.position` and `self.buffered`.
/* FP:opaque.rs-0075 */         self.flushed.debug_strict_add(self.buffered)
/* FP:opaque.rs-0076 */     }
/* FP:opaque.rs-0077 */ 
/* FP:opaque.rs-0078 */     #[cold]
/* FP:opaque.rs-0079 */     #[inline(never)]
/* FP:opaque.rs-0080 */     pub fn flush(&mut self) {
/* FP:opaque.rs-0081 */         #[cfg(debug_assertions)]
/* FP:opaque.rs-0082 */         {
/* FP:opaque.rs-0083 */             self.finished = false;
/* FP:opaque.rs-0084 */         }
/* FP:opaque.rs-0085 */         if self.res.is_ok() {
/* FP:opaque.rs-0086 */             self.res = self.file.write_all(&self.buf[..self.buffered]);
/* FP:opaque.rs-0087 */         }
/* FP:opaque.rs-0088 */         self.flushed += self.buffered;
/* FP:opaque.rs-0089 */         self.buffered = 0;
/* FP:opaque.rs-0090 */     }
/* FP:opaque.rs-0091 */ 
/* FP:opaque.rs-0092 */     #[inline]
/* FP:opaque.rs-0093 */     pub fn file(&self) -> &File {
/* FP:opaque.rs-0094 */         &self.file
/* FP:opaque.rs-0095 */     }
/* FP:opaque.rs-0096 */ 
/* FP:opaque.rs-0097 */     #[inline]
/* FP:opaque.rs-0098 */     pub fn path(&self) -> &Path {
/* FP:opaque.rs-0099 */         &self.path
/* FP:opaque.rs-0100 */     }
/* FP:opaque.rs-0101 */ 
/* FP:opaque.rs-0102 */     #[inline]
/* FP:opaque.rs-0103 */     fn buffer_empty(&mut self) -> &mut [u8] {
/* FP:opaque.rs-0104 */         // SAFETY: self.buffered is inbounds as an invariant of the type
/* FP:opaque.rs-0105 */         unsafe { self.buf.get_unchecked_mut(self.buffered..) }
/* FP:opaque.rs-0106 */     }
/* FP:opaque.rs-0107 */ 
/* FP:opaque.rs-0108 */     #[cold]
/* FP:opaque.rs-0109 */     #[inline(never)]
/* FP:opaque.rs-0110 */     fn write_all_cold_path(&mut self, buf: &[u8]) {
/* FP:opaque.rs-0111 */         self.flush();
/* FP:opaque.rs-0112 */         if let Some(dest) = self.buf.get_mut(..buf.len()) {
/* FP:opaque.rs-0113 */             dest.copy_from_slice(buf);
/* FP:opaque.rs-0114 */             self.buffered += buf.len();
/* FP:opaque.rs-0115 */         } else {
/* FP:opaque.rs-0116 */             if self.res.is_ok() {
/* FP:opaque.rs-0117 */                 self.res = self.file.write_all(buf);
/* FP:opaque.rs-0118 */             }
/* FP:opaque.rs-0119 */             self.flushed += buf.len();
/* FP:opaque.rs-0120 */         }
/* FP:opaque.rs-0121 */     }
/* FP:opaque.rs-0122 */ 
/* FP:opaque.rs-0123 */     #[inline]
/* FP:opaque.rs-0124 */     fn write_all(&mut self, buf: &[u8]) {
/* FP:opaque.rs-0125 */         #[cfg(debug_assertions)]
/* FP:opaque.rs-0126 */         {
/* FP:opaque.rs-0127 */             self.finished = false;
/* FP:opaque.rs-0128 */         }
/* FP:opaque.rs-0129 */         if let Some(dest) = self.buffer_empty().get_mut(..buf.len()) {
/* FP:opaque.rs-0130 */             dest.copy_from_slice(buf);
/* FP:opaque.rs-0131 */             self.buffered = self.buffered.debug_strict_add(buf.len());
/* FP:opaque.rs-0132 */         } else {
/* FP:opaque.rs-0133 */             self.write_all_cold_path(buf);
/* FP:opaque.rs-0134 */         }
/* FP:opaque.rs-0135 */     }
/* FP:opaque.rs-0136 */ 
/* FP:opaque.rs-0137 */     /// Write up to `N` bytes to this encoder.
/* FP:opaque.rs-0138 */     ///
/* FP:opaque.rs-0139 */     /// This function can be used to avoid the overhead of calling memcpy for writes that
/* FP:opaque.rs-0140 */     /// have runtime-variable length, but are small and have a small fixed upper bound.
/* FP:opaque.rs-0141 */     ///
/* FP:opaque.rs-0142 */     /// This can be used to do in-place encoding as is done for leb128 (without this function
/* FP:opaque.rs-0143 */     /// we would need to write to a temporary buffer then memcpy into the encoder), and it can
/* FP:opaque.rs-0144 */     /// also be used to implement the varint scheme we use for rmeta and dep graph encoding,
/* FP:opaque.rs-0145 */     /// where we only want to encode the first few bytes of an integer. Copying in the whole
/* FP:opaque.rs-0146 */     /// integer then only advancing the encoder state for the few bytes we care about is more
/* FP:opaque.rs-0147 */     /// efficient than calling [`FileEncoder::write_all`], because variable-size copies are
/* FP:opaque.rs-0148 */     /// always lowered to `memcpy`, which has overhead and contains a lot of logic we can bypass
/* FP:opaque.rs-0149 */     /// with this function. Note that common architectures support fixed-size writes up to 8 bytes
/* FP:opaque.rs-0150 */     /// with one instruction, so while this does in some sense do wasted work, we come out ahead.
/* FP:opaque.rs-0151 */     #[inline]
/* FP:opaque.rs-0152 */     pub fn write_with<const N: usize>(&mut self, visitor: impl FnOnce(&mut [u8; N]) -> usize) {
/* FP:opaque.rs-0153 */         #[cfg(debug_assertions)]
/* FP:opaque.rs-0154 */         {
/* FP:opaque.rs-0155 */             self.finished = false;
/* FP:opaque.rs-0156 */         }
/* FP:opaque.rs-0157 */         let flush_threshold = const { BUF_SIZE.checked_sub(N).unwrap() };
/* FP:opaque.rs-0158 */         if std::intrinsics::unlikely(self.buffered > flush_threshold) {
/* FP:opaque.rs-0159 */             self.flush();
/* FP:opaque.rs-0160 */         }
/* FP:opaque.rs-0161 */         // SAFETY: We checked above that N < self.buffer_empty().len(),
/* FP:opaque.rs-0162 */         // and if isn't, flush ensures that our empty buffer is now BUF_SIZE.
/* FP:opaque.rs-0163 */         // We produce a post-mono error if N > BUF_SIZE.
/* FP:opaque.rs-0164 */         let buf = unsafe { self.buffer_empty().first_chunk_mut::<N>().unwrap_unchecked() };
/* FP:opaque.rs-0165 */         let written = visitor(buf);
/* FP:opaque.rs-0166 */         // We have to ensure that an errant visitor cannot cause self.buffered to exceed BUF_SIZE.
/* FP:opaque.rs-0167 */         if written > N {
/* FP:opaque.rs-0168 */             Self::panic_invalid_write::<N>(written);
/* FP:opaque.rs-0169 */         }
/* FP:opaque.rs-0170 */         self.buffered = self.buffered.debug_strict_add(written);
/* FP:opaque.rs-0171 */     }
/* FP:opaque.rs-0172 */ 
/* FP:opaque.rs-0173 */     #[cold]
/* FP:opaque.rs-0174 */     #[inline(never)]
/* FP:opaque.rs-0175 */     fn panic_invalid_write<const N: usize>(written: usize) {
/* FP:opaque.rs-0176 */         panic!("FileEncoder::write_with::<{N}> cannot be used to write {written} bytes");
/* FP:opaque.rs-0177 */     }
/* FP:opaque.rs-0178 */ 
/* FP:opaque.rs-0179 */     /// Helper for calls where [`FileEncoder::write_with`] always writes the whole array.
/* FP:opaque.rs-0180 */     #[inline]
/* FP:opaque.rs-0181 */     pub fn write_array<const N: usize>(&mut self, buf: [u8; N]) {
/* FP:opaque.rs-0182 */         self.write_with(|dest| {
/* FP:opaque.rs-0183 */             *dest = buf;
/* FP:opaque.rs-0184 */             N
/* FP:opaque.rs-0185 */         })
/* FP:opaque.rs-0186 */     }
/* FP:opaque.rs-0187 */ 
/* FP:opaque.rs-0188 */     pub fn finish(&mut self) -> FileEncodeResult {
/* FP:opaque.rs-0189 */         self.write_all(MAGIC_END_BYTES);
/* FP:opaque.rs-0190 */         self.flush();
/* FP:opaque.rs-0191 */         #[cfg(debug_assertions)]
/* FP:opaque.rs-0192 */         {
/* FP:opaque.rs-0193 */             self.finished = true;
/* FP:opaque.rs-0194 */         }
/* FP:opaque.rs-0195 */         match std::mem::replace(&mut self.res, Ok(())) {
/* FP:opaque.rs-0196 */             Ok(()) => Ok(self.position()),
/* FP:opaque.rs-0197 */             Err(e) => Err((self.path.clone(), e)),
/* FP:opaque.rs-0198 */         }
/* FP:opaque.rs-0199 */     }
/* FP:opaque.rs-0200 */ }
/* FP:opaque.rs-0201 */ 
/* FP:opaque.rs-0202 */ #[cfg(debug_assertions)]
/* FP:opaque.rs-0203 */ impl Drop for FileEncoder {
/* FP:opaque.rs-0204 */     fn drop(&mut self) {
/* FP:opaque.rs-0205 */         if !std::thread::panicking() {
/* FP:opaque.rs-0206 */             assert!(self.finished);
/* FP:opaque.rs-0207 */         }
/* FP:opaque.rs-0208 */     }
/* FP:opaque.rs-0209 */ }
/* FP:opaque.rs-0210 */ 
/* FP:opaque.rs-0211 */ macro_rules! write_leb128 {
/* FP:opaque.rs-0212 */     ($this_fn:ident, $int_ty:ty, $write_leb_fn:ident) => {
/* FP:opaque.rs-0213 */         #[inline]
/* FP:opaque.rs-0214 */         fn $this_fn(&mut self, v: $int_ty) {
/* FP:opaque.rs-0215 */             self.write_with(|buf| leb128::$write_leb_fn(buf, v))
/* FP:opaque.rs-0216 */         }
/* FP:opaque.rs-0217 */     };
/* FP:opaque.rs-0218 */ }
/* FP:opaque.rs-0219 */ 
/* FP:opaque.rs-0220 */ impl Encoder for FileEncoder {
/* FP:opaque.rs-0221 */     write_leb128!(emit_usize, usize, write_usize_leb128);
/* FP:opaque.rs-0222 */     write_leb128!(emit_u128, u128, write_u128_leb128);
/* FP:opaque.rs-0223 */     write_leb128!(emit_u64, u64, write_u64_leb128);
/* FP:opaque.rs-0224 */     write_leb128!(emit_u32, u32, write_u32_leb128);
/* FP:opaque.rs-0225 */ 
/* FP:opaque.rs-0226 */     #[inline]
/* FP:opaque.rs-0227 */     fn emit_u16(&mut self, v: u16) {
/* FP:opaque.rs-0228 */         self.write_array(v.to_le_bytes());
/* FP:opaque.rs-0229 */     }
/* FP:opaque.rs-0230 */ 
/* FP:opaque.rs-0231 */     #[inline]
/* FP:opaque.rs-0232 */     fn emit_u8(&mut self, v: u8) {
/* FP:opaque.rs-0233 */         self.write_array([v]);
/* FP:opaque.rs-0234 */     }
/* FP:opaque.rs-0235 */ 
/* FP:opaque.rs-0236 */     write_leb128!(emit_isize, isize, write_isize_leb128);
/* FP:opaque.rs-0237 */     write_leb128!(emit_i128, i128, write_i128_leb128);
/* FP:opaque.rs-0238 */     write_leb128!(emit_i64, i64, write_i64_leb128);
/* FP:opaque.rs-0239 */     write_leb128!(emit_i32, i32, write_i32_leb128);
/* FP:opaque.rs-0240 */ 
/* FP:opaque.rs-0241 */     #[inline]
/* FP:opaque.rs-0242 */     fn emit_i16(&mut self, v: i16) {
/* FP:opaque.rs-0243 */         self.write_array(v.to_le_bytes());
/* FP:opaque.rs-0244 */     }
/* FP:opaque.rs-0245 */ 
/* FP:opaque.rs-0246 */     #[inline]
/* FP:opaque.rs-0247 */     fn emit_raw_bytes(&mut self, s: &[u8]) {
/* FP:opaque.rs-0248 */         self.write_all(s);
/* FP:opaque.rs-0249 */     }
/* FP:opaque.rs-0250 */ }
/* FP:opaque.rs-0251 */ 
/* FP:opaque.rs-0252 */ // -----------------------------------------------------------------------------
/* FP:opaque.rs-0253 */ // Decoder
/* FP:opaque.rs-0254 */ // -----------------------------------------------------------------------------
/* FP:opaque.rs-0255 */ 
/* FP:opaque.rs-0256 */ // Conceptually, `MemDecoder` wraps a `&[u8]` with a cursor into it that is always valid.
/* FP:opaque.rs-0257 */ // This is implemented with three pointers, two which represent the original slice and a
/* FP:opaque.rs-0258 */ // third that is our cursor.
/* FP:opaque.rs-0259 */ // It is an invariant of this type that start <= current <= end.
/* FP:opaque.rs-0260 */ // Additionally, the implementation of this type never modifies start and end.
/* FP:opaque.rs-0261 */ pub struct MemDecoder<'a> {
/* FP:opaque.rs-0262 */     start: *const u8,
/* FP:opaque.rs-0263 */     current: *const u8,
/* FP:opaque.rs-0264 */     end: *const u8,
/* FP:opaque.rs-0265 */     _marker: PhantomData<&'a u8>,
/* FP:opaque.rs-0266 */ }
/* FP:opaque.rs-0267 */ 
/* FP:opaque.rs-0268 */ impl<'a> MemDecoder<'a> {
/* FP:opaque.rs-0269 */     #[inline]
/* FP:opaque.rs-0270 */     pub fn new(data: &'a [u8], position: usize) -> Result<MemDecoder<'a>, ()> {
/* FP:opaque.rs-0271 */         let data = data.strip_suffix(MAGIC_END_BYTES).ok_or(())?;
/* FP:opaque.rs-0272 */         let Range { start, end } = data.as_ptr_range();
/* FP:opaque.rs-0273 */         Ok(MemDecoder { start, current: data[position..].as_ptr(), end, _marker: PhantomData })
/* FP:opaque.rs-0274 */     }
/* FP:opaque.rs-0275 */ 
/* FP:opaque.rs-0276 */     #[inline]
/* FP:opaque.rs-0277 */     pub fn split_at(&self, position: usize) -> MemDecoder<'a> {
/* FP:opaque.rs-0278 */         assert!(position <= self.len());
/* FP:opaque.rs-0279 */         // SAFETY: We checked above that this offset is within the original slice
/* FP:opaque.rs-0280 */         let current = unsafe { self.start.add(position) };
/* FP:opaque.rs-0281 */         MemDecoder { start: self.start, current, end: self.end, _marker: PhantomData }
/* FP:opaque.rs-0282 */     }
/* FP:opaque.rs-0283 */ 
/* FP:opaque.rs-0284 */     #[inline]
/* FP:opaque.rs-0285 */     pub fn len(&self) -> usize {
/* FP:opaque.rs-0286 */         // SAFETY: This recovers the length of the original slice, only using members we never modify.
/* FP:opaque.rs-0287 */         unsafe { self.end.offset_from_unsigned(self.start) }
/* FP:opaque.rs-0288 */     }
/* FP:opaque.rs-0289 */ 
/* FP:opaque.rs-0290 */     #[inline]
/* FP:opaque.rs-0291 */     pub fn remaining(&self) -> usize {
/* FP:opaque.rs-0292 */         // SAFETY: This type guarantees current <= end.
/* FP:opaque.rs-0293 */         unsafe { self.end.offset_from_unsigned(self.current) }
/* FP:opaque.rs-0294 */     }
/* FP:opaque.rs-0295 */ 
/* FP:opaque.rs-0296 */     #[cold]
/* FP:opaque.rs-0297 */     #[inline(never)]
/* FP:opaque.rs-0298 */     fn decoder_exhausted() -> ! {
/* FP:opaque.rs-0299 */         panic!("MemDecoder exhausted")
/* FP:opaque.rs-0300 */     }
/* FP:opaque.rs-0301 */ 
/* FP:opaque.rs-0302 */     #[inline]
/* FP:opaque.rs-0303 */     pub fn read_array<const N: usize>(&mut self) -> [u8; N] {
/* FP:opaque.rs-0304 */         self.read_raw_bytes(N).try_into().unwrap()
/* FP:opaque.rs-0305 */     }
/* FP:opaque.rs-0306 */ 
/* FP:opaque.rs-0307 */     /// While we could manually expose manipulation of the decoder position,
/* FP:opaque.rs-0308 */     /// all current users of that method would need to reset the position later,
/* FP:opaque.rs-0309 */     /// incurring the bounds check of set_position twice.
/* FP:opaque.rs-0310 */     #[inline]
/* FP:opaque.rs-0311 */     pub fn with_position<F, T>(&mut self, pos: usize, func: F) -> T
/* FP:opaque.rs-0312 */     where
/* FP:opaque.rs-0313 */         F: Fn(&mut MemDecoder<'a>) -> T,
/* FP:opaque.rs-0314 */     {
/* FP:opaque.rs-0315 */         struct SetOnDrop<'a, 'guarded> {
/* FP:opaque.rs-0316 */             decoder: &'guarded mut MemDecoder<'a>,
/* FP:opaque.rs-0317 */             current: *const u8,
/* FP:opaque.rs-0318 */         }
/* FP:opaque.rs-0319 */         impl Drop for SetOnDrop<'_, '_> {
/* FP:opaque.rs-0320 */             fn drop(&mut self) {
/* FP:opaque.rs-0321 */                 self.decoder.current = self.current;
/* FP:opaque.rs-0322 */             }
/* FP:opaque.rs-0323 */         }
/* FP:opaque.rs-0324 */ 
/* FP:opaque.rs-0325 */         if pos >= self.len() {
/* FP:opaque.rs-0326 */             Self::decoder_exhausted();
/* FP:opaque.rs-0327 */         }
/* FP:opaque.rs-0328 */         let previous = self.current;
/* FP:opaque.rs-0329 */         // SAFETY: We just checked if this add is in-bounds above.
/* FP:opaque.rs-0330 */         unsafe {
/* FP:opaque.rs-0331 */             self.current = self.start.add(pos);
/* FP:opaque.rs-0332 */         }
/* FP:opaque.rs-0333 */         let guard = SetOnDrop { current: previous, decoder: self };
/* FP:opaque.rs-0334 */         func(guard.decoder)
/* FP:opaque.rs-0335 */     }
/* FP:opaque.rs-0336 */ }
/* FP:opaque.rs-0337 */ 
/* FP:opaque.rs-0338 */ macro_rules! read_leb128 {
/* FP:opaque.rs-0339 */     ($this_fn:ident, $int_ty:ty, $read_leb_fn:ident) => {
/* FP:opaque.rs-0340 */         #[inline]
/* FP:opaque.rs-0341 */         fn $this_fn(&mut self) -> $int_ty {
/* FP:opaque.rs-0342 */             leb128::$read_leb_fn(self)
/* FP:opaque.rs-0343 */         }
/* FP:opaque.rs-0344 */     };
/* FP:opaque.rs-0345 */ }
/* FP:opaque.rs-0346 */ 
/* FP:opaque.rs-0347 */ impl<'a> Decoder for MemDecoder<'a> {
/* FP:opaque.rs-0348 */     read_leb128!(read_usize, usize, read_usize_leb128);
/* FP:opaque.rs-0349 */     read_leb128!(read_u128, u128, read_u128_leb128);
/* FP:opaque.rs-0350 */     read_leb128!(read_u64, u64, read_u64_leb128);
/* FP:opaque.rs-0351 */     read_leb128!(read_u32, u32, read_u32_leb128);
/* FP:opaque.rs-0352 */ 
/* FP:opaque.rs-0353 */     #[inline]
/* FP:opaque.rs-0354 */     fn read_u16(&mut self) -> u16 {
/* FP:opaque.rs-0355 */         u16::from_le_bytes(self.read_array())
/* FP:opaque.rs-0356 */     }
/* FP:opaque.rs-0357 */ 
/* FP:opaque.rs-0358 */     #[inline]
/* FP:opaque.rs-0359 */     fn read_u8(&mut self) -> u8 {
/* FP:opaque.rs-0360 */         if self.current == self.end {
/* FP:opaque.rs-0361 */             Self::decoder_exhausted();
/* FP:opaque.rs-0362 */         }
/* FP:opaque.rs-0363 */         // SAFETY: This type guarantees current <= end, and we just checked current == end.
/* FP:opaque.rs-0364 */         unsafe {
/* FP:opaque.rs-0365 */             let byte = *self.current;
/* FP:opaque.rs-0366 */             self.current = self.current.add(1);
/* FP:opaque.rs-0367 */             byte
/* FP:opaque.rs-0368 */         }
/* FP:opaque.rs-0369 */     }
/* FP:opaque.rs-0370 */ 
/* FP:opaque.rs-0371 */     read_leb128!(read_isize, isize, read_isize_leb128);
/* FP:opaque.rs-0372 */     read_leb128!(read_i128, i128, read_i128_leb128);
/* FP:opaque.rs-0373 */     read_leb128!(read_i64, i64, read_i64_leb128);
/* FP:opaque.rs-0374 */     read_leb128!(read_i32, i32, read_i32_leb128);
/* FP:opaque.rs-0375 */ 
/* FP:opaque.rs-0376 */     #[inline]
/* FP:opaque.rs-0377 */     fn read_i16(&mut self) -> i16 {
/* FP:opaque.rs-0378 */         i16::from_le_bytes(self.read_array())
/* FP:opaque.rs-0379 */     }
/* FP:opaque.rs-0380 */ 
/* FP:opaque.rs-0381 */     #[inline]
/* FP:opaque.rs-0382 */     fn read_raw_bytes(&mut self, bytes: usize) -> &'a [u8] {
/* FP:opaque.rs-0383 */         if bytes > self.remaining() {
/* FP:opaque.rs-0384 */             Self::decoder_exhausted();
/* FP:opaque.rs-0385 */         }
/* FP:opaque.rs-0386 */         // SAFETY: We just checked if this range is in-bounds above.
/* FP:opaque.rs-0387 */         unsafe {
/* FP:opaque.rs-0388 */             let slice = std::slice::from_raw_parts(self.current, bytes);
/* FP:opaque.rs-0389 */             self.current = self.current.add(bytes);
/* FP:opaque.rs-0390 */             slice
/* FP:opaque.rs-0391 */         }
/* FP:opaque.rs-0392 */     }
/* FP:opaque.rs-0393 */ 
/* FP:opaque.rs-0394 */     #[inline]
/* FP:opaque.rs-0395 */     fn peek_byte(&self) -> u8 {
/* FP:opaque.rs-0396 */         if self.current == self.end {
/* FP:opaque.rs-0397 */             Self::decoder_exhausted();
/* FP:opaque.rs-0398 */         }
/* FP:opaque.rs-0399 */         // SAFETY: This type guarantees current is inbounds or one-past-the-end, which is end.
/* FP:opaque.rs-0400 */         // Since we just checked current == end, the current pointer must be inbounds.
/* FP:opaque.rs-0401 */         unsafe { *self.current }
/* FP:opaque.rs-0402 */     }
/* FP:opaque.rs-0403 */ 
/* FP:opaque.rs-0404 */     #[inline]
/* FP:opaque.rs-0405 */     fn position(&self) -> usize {
/* FP:opaque.rs-0406 */         // SAFETY: This type guarantees start <= current
/* FP:opaque.rs-0407 */         unsafe { self.current.offset_from_unsigned(self.start) }
/* FP:opaque.rs-0408 */     }
/* FP:opaque.rs-0409 */ }
/* FP:opaque.rs-0410 */ 
/* FP:opaque.rs-0411 */ // Specializations for contiguous byte sequences follow. The default implementations for slices
/* FP:opaque.rs-0412 */ // encode and decode each element individually. This isn't necessary for `u8` slices when using
/* FP:opaque.rs-0413 */ // opaque encoders and decoders, because each `u8` is unchanged by encoding and decoding.
/* FP:opaque.rs-0414 */ // Therefore, we can use more efficient implementations that process the entire sequence at once.
/* FP:opaque.rs-0415 */ 
/* FP:opaque.rs-0416 */ // Specialize encoding byte slices. This specialization also applies to encoding `Vec<u8>`s, etc.,
/* FP:opaque.rs-0417 */ // since the default implementations call `encode` on their slices internally.
/* FP:opaque.rs-0418 */ impl Encodable<FileEncoder> for [u8] {
/* FP:opaque.rs-0419 */     fn encode(&self, e: &mut FileEncoder) {
/* FP:opaque.rs-0420 */         Encoder::emit_usize(e, self.len());
/* FP:opaque.rs-0421 */         e.emit_raw_bytes(self);
/* FP:opaque.rs-0422 */     }
/* FP:opaque.rs-0423 */ }
/* FP:opaque.rs-0424 */ 
/* FP:opaque.rs-0425 */ // Specialize decoding `Vec<u8>`. This specialization also applies to decoding `Box<[u8]>`s, etc.,
/* FP:opaque.rs-0426 */ // since the default implementations call `decode` to produce a `Vec<u8>` internally.
/* FP:opaque.rs-0427 */ impl<'a> Decodable<MemDecoder<'a>> for Vec<u8> {
/* FP:opaque.rs-0428 */     fn decode(d: &mut MemDecoder<'a>) -> Self {
/* FP:opaque.rs-0429 */         let len = Decoder::read_usize(d);
/* FP:opaque.rs-0430 */         d.read_raw_bytes(len).to_owned()
/* FP:opaque.rs-0431 */     }
/* FP:opaque.rs-0432 */ }
/* FP:opaque.rs-0433 */ 
/* FP:opaque.rs-0434 */ /// An integer that will always encode to 8 bytes.
/* FP:opaque.rs-0435 */ pub struct IntEncodedWithFixedSize(pub u64);
/* FP:opaque.rs-0436 */ 
/* FP:opaque.rs-0437 */ impl IntEncodedWithFixedSize {
/* FP:opaque.rs-0438 */     pub const ENCODED_SIZE: usize = 8;
/* FP:opaque.rs-0439 */ }
/* FP:opaque.rs-0440 */ 
/* FP:opaque.rs-0441 */ impl Encodable<FileEncoder> for IntEncodedWithFixedSize {
/* FP:opaque.rs-0442 */     #[inline]
/* FP:opaque.rs-0443 */     fn encode(&self, e: &mut FileEncoder) {
/* FP:opaque.rs-0444 */         let start_pos = e.position();
/* FP:opaque.rs-0445 */         e.write_array(self.0.to_le_bytes());
/* FP:opaque.rs-0446 */         let end_pos = e.position();
/* FP:opaque.rs-0447 */         debug_assert_eq!((end_pos - start_pos), IntEncodedWithFixedSize::ENCODED_SIZE);
/* FP:opaque.rs-0448 */     }
/* FP:opaque.rs-0449 */ }
/* FP:opaque.rs-0450 */ 
/* FP:opaque.rs-0451 */ impl<'a> Decodable<MemDecoder<'a>> for IntEncodedWithFixedSize {
/* FP:opaque.rs-0452 */     #[inline]
/* FP:opaque.rs-0453 */     fn decode(decoder: &mut MemDecoder<'a>) -> IntEncodedWithFixedSize {
/* FP:opaque.rs-0454 */         let bytes = decoder.read_array::<{ IntEncodedWithFixedSize::ENCODED_SIZE }>();
/* FP:opaque.rs-0455 */         IntEncodedWithFixedSize(u64::from_le_bytes(bytes))
/* FP:opaque.rs-0456 */     }
/* FP:opaque.rs-0457 */ }
/* FP:opaque.rs-0458 */ 
/* FP:opaque.rs-0459 */ #[cfg(test)]