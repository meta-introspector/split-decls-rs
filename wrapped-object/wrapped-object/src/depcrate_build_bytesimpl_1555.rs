// Generated macro for impl_1555 (impl)
macro_rules! Depcrate_build_bytesimpl_1555 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1555"}
// Dependencies: {}
impl < 'a > From < & 'a str > for ByteString < 'a > { fn from (s : & 'a str) -> Self { ByteString (Cow :: Borrowed (s . as_bytes ())) } }
};
}
