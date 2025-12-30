// Generated macro for impl_322 (impl)
macro_rules! Depcrate_dsaimpl_322 {
() => {
// Module: crate::dsa
// Provides: {"impl_322"}
// Dependencies: {}
impl DsaSigRef { to_der ! { # [doc = " Serializes the DSA signature into a DER-encoded `DSASignature` structure."] # [corresponds (i2d_DSA_SIG)] to_der , ffi :: i2d_DSA_SIG } # [doc = " Returns internal component `r` of an `DsaSig`."] # [corresponds (DSA_SIG_get0)] pub fn r (& self) -> & BigNumRef { unsafe { let mut r = ptr :: null () ; DSA_SIG_get0 (self . as_ptr () , & mut r , ptr :: null_mut ()) ; BigNumRef :: from_const_ptr (r) } } # [doc = " Returns internal component `s` of an `DsaSig`."] # [corresponds (DSA_SIG_get0)] pub fn s (& self) -> & BigNumRef { unsafe { let mut s = ptr :: null () ; DSA_SIG_get0 (self . as_ptr () , ptr :: null_mut () , & mut s) ; BigNumRef :: from_const_ptr (s) } } }
};
}
