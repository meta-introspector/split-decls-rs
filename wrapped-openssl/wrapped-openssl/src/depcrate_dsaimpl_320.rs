// Generated macro for impl_320 (impl)
macro_rules! Depcrate_dsaimpl_320 {
() => {
// Module: crate::dsa
// Provides: {"impl_320"}
// Dependencies: {}
impl DsaSig { # [doc = " Returns a new `DsaSig` by setting the `r` and `s` values associated with an DSA signature."] # [corresponds (DSA_SIG_set0)] pub fn from_private_components (r : BigNum , s : BigNum) -> Result < Self , ErrorStack > { unsafe { let sig = cvt_p (ffi :: DSA_SIG_new ()) ? ; DSA_SIG_set0 (sig , r . as_ptr () , s . as_ptr ()) ; mem :: forget ((r , s)) ; Ok (DsaSig :: from_ptr (sig)) } } from_der ! { # [doc = " Decodes a DER-encoded DSA signature."] # [corresponds (d2i_DSA_SIG)] from_der , DsaSig , ffi :: d2i_DSA_SIG } }
};
}
