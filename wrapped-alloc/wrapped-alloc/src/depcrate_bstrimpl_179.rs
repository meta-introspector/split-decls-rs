// Generated macro for impl_179 (impl)
macro_rules! Depcrate_bstrimpl_179 {
() => {
// Module: crate::bstr
// Provides: {"impl_179"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl fmt :: Debug for ByteString { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . as_bytestr () , f) } }
};
}
