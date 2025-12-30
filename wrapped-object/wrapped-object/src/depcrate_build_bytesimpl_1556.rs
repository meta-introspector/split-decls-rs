// Generated macro for impl_1556 (impl)
macro_rules! Depcrate_build_bytesimpl_1556 {
() => {
// Module: crate::build::bytes
// Provides: {"impl_1556"}
// Dependencies: {}
impl < 'a > fmt :: Debug for ByteString < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "\"{}\"" , String :: from_utf8_lossy (& self . 0)) } }
};
}
