// Generated macro for impl_1593 (impl)
macro_rules! Depcrate_x509impl_1593 {
() => {
// Module: crate::x509
// Provides: {"impl_1593"}
// Dependencies: {}
impl < 'a > CrlStatus < 'a > { unsafe fn from_ffi_status (status : c_int , revoked_entry : * mut ffi :: X509_REVOKED ,) -> CrlStatus < 'a > { match status { 0 => CrlStatus :: NotRevoked , 1 => { assert ! (! revoked_entry . is_null ()) ; CrlStatus :: Revoked (X509RevokedRef :: from_ptr (revoked_entry)) } 2 => { assert ! (! revoked_entry . is_null ()) ; CrlStatus :: RemoveFromCrl (X509RevokedRef :: from_ptr (revoked_entry)) } _ => unreachable ! ("{}" , "X509_CRL_get0_by_{{serial,cert}} should only return 0, 1, or 2.") , } } }
};
}
