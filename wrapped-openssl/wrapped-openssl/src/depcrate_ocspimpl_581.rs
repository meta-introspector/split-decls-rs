// Generated macro for impl_581 (impl)
macro_rules! Depcrate_ocspimpl_581 {
() => {
// Module: crate::ocsp
// Provides: {"impl_581"}
// Dependencies: {}
impl OcspStatus < '_ > { # [doc = " Returns the time at which this revocation check expires."] # [doc = ""] # [doc = " Returns `None` if the OCSP response does not include a `next_update`"] # [doc = " field."] pub fn next_update (& self) -> Option < & Asn1GeneralizedTimeRef > { self . next_update_opt } # [doc = " Checks validity of the `this_update` and `next_update` fields."] # [doc = ""] # [doc = " The `nsec` parameter specifies an amount of slack time that will be used when comparing"] # [doc = " those times with the current time to account for delays and clock skew."] # [doc = ""] # [doc = " The `maxsec` parameter limits the maximum age of the `this_update` parameter to prohibit"] # [doc = " very old responses."] # [corresponds (OCSP_check_validity)] pub fn check_validity (& self , nsec : u32 , maxsec : Option < u32 >) -> Result < () , ErrorStack > { let next_update_ptr = self . next_update_opt . map (| t | t . as_ptr ()) . unwrap_or (ptr :: null_mut ()) ; unsafe { cvt (ffi :: OCSP_check_validity (self . this_update . as_ptr () , next_update_ptr , nsec as c_long , maxsec . map (| n | n as c_long) . unwrap_or (- 1) ,)) . map (| _ | ()) } } }
};
}
