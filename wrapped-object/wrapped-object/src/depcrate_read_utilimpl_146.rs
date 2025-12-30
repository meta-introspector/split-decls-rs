// Generated macro for impl_146 (impl)
macro_rules! Depcrate_read_utilimpl_146 {
() => {
// Module: crate::read::util
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'data > fmt :: Debug for ByteString < 'data > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "\"{}\"" , String :: from_utf8_lossy (self . 0)) } }
};
}
