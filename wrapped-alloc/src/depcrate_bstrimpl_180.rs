// Generated macro for impl_180 (impl)
macro_rules! Depcrate_bstrimpl_180 {
() => {
// Module: crate::bstr
// Provides: {"impl_180"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl fmt :: Display for ByteString { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self . as_bytestr () , f) } }
};
}
