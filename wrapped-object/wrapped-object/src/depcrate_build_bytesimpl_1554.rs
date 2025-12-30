// Generated macro for impl_1554 (impl)
macro_rules! Depcrate_build_bytesimpl_1554 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1554"}
// Dependencies: {}
impl < 'a > From < Vec < u8 > > for ByteString < 'a > { fn from (bytes : Vec < u8 >) -> Self { ByteString (Cow :: Owned (bytes)) } }
};
}
