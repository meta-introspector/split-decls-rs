// Generated macro for impl_1553 (impl)
macro_rules! Depcrate_build_bytesimpl_1553 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1553"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for ByteString < 'a > { fn from (bytes : & 'a [u8]) -> Self { ByteString (Cow :: Borrowed (bytes)) } }
};
}
