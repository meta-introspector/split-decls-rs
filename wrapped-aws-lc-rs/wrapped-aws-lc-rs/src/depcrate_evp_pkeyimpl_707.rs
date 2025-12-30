// Generated macro for impl_707 (impl)
macro_rules! Depcrate_evp_pkeyimpl_707 {
() => {
// Module: crate::evp_pkey
// Provides: {"impl_707"}
// Dependencies: {}
impl PartialEq < Self > for LcPtr < EVP_PKEY > { # [doc = " Only compares params and public key"] fn eq (& self , other : & Self) -> bool { 1 == unsafe { EVP_PKEY_cmp (* self . as_const () , * other . as_const ()) } } }
};
}
