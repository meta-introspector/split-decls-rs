// Generated macro for impl_169 (impl)
macro_rules! Depcrate_hpack_headerimpl_169 {
() => {
// Module: crate::hpack::header
// Provides: {"impl_169"}
// Dependencies: {}
impl BytesStr { pub (crate) const fn from_static (value : & 'static str) -> Self { BytesStr (Bytes :: from_static (value . as_bytes ())) } pub (crate) fn from (value : & str) -> Self { BytesStr (Bytes :: copy_from_slice (value . as_bytes ())) } # [doc (hidden)] pub fn try_from (bytes : Bytes) -> Result < Self , std :: str :: Utf8Error > { std :: str :: from_utf8 (bytes . as_ref ()) ? ; Ok (BytesStr (bytes)) } pub (crate) fn as_str (& self) -> & str { unsafe { std :: str :: from_utf8_unchecked (self . 0 . as_ref ()) } } pub (crate) fn into_inner (self) -> Bytes { self . 0 } }
};
}
