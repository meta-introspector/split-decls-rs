// Generated macro for impl_594 (impl)
macro_rules! Depcrate_byte_strimpl_594 {
() => {
// Module: crate::byte_str
// Provides: {"impl_594"}
// Dependencies: {}
impl < 'a > From < & 'a str > for ByteStr { # [inline] fn from (src : & 'a str) -> ByteStr { ByteStr { bytes : Bytes :: copy_from_slice (src . as_bytes ()) , } } }
};
}
