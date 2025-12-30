// Generated macro for impl_57 (impl)
macro_rules! Depcrate_private_keyimpl_57 {
() => {
// Module: crate::private_key
// Provides: {"impl_57"}
// Dependencies: {}
impl fmt :: Debug for RsaPrivateKey < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RsaPrivateKey") . field ("version" , & self . version ()) . field ("modulus" , & self . modulus) . field ("public_exponent" , & self . public_exponent) . finish_non_exhaustive () } }
};
}
