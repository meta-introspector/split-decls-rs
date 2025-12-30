// Generated macro for impl_1557 (impl)
macro_rules! Depcrate_build_bytesimpl_1557 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1557"}
// Dependencies: {}
impl < 'a > fmt :: Display for ByteString < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{}" , String :: from_utf8_lossy (& self . 0)) } }
};
}
