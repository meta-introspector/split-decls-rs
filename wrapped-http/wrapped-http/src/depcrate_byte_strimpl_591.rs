// Generated macro for impl_591 (impl)
macro_rules! Depcrate_byte_strimpl_591 {
() => {
// Module: crate::byte_str
// Provides: {"impl_591"}
// Dependencies: {}
impl ByteStr { # [inline] pub fn new () -> ByteStr { ByteStr { bytes : Bytes :: new () , } } # [inline] pub const fn from_static (val : & 'static str) -> ByteStr { ByteStr { bytes : Bytes :: from_static (val . as_bytes ()) , } } # [inline] # [doc = " ## Panics"] # [doc = " In a debug build this will panic if `bytes` is not valid UTF-8."] # [doc = ""] # [doc = " ## Safety"] # [doc = " `bytes` must contain valid UTF-8. In a release build it is undefined"] # [doc = " behavior to call this with `bytes` that is not valid UTF-8."] pub unsafe fn from_utf8_unchecked (bytes : Bytes) -> ByteStr { if cfg ! (debug_assertions) { match str :: from_utf8 (& bytes) { Ok (_) => () , Err (err) => panic ! ("ByteStr::from_utf8_unchecked() with invalid bytes; error = {}, bytes = {:?}" , err , bytes) , } } ByteStr { bytes } } pub (crate) fn from_utf8 (bytes : Bytes) -> Result < ByteStr , std :: str :: Utf8Error > { str :: from_utf8 (& bytes) ? ; Ok (ByteStr { bytes }) } }
};
}
