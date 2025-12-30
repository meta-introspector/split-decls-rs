// Generated macro for impl_1541 (impl)
macro_rules! Depcrate_build_bytesimpl_1541 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1541"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for Bytes < 'a > { fn from (bytes : & 'a [u8]) -> Self { Bytes (Cow :: Borrowed (bytes)) } }
};
}
