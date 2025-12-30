// Generated macro for impl_713 (impl)
macro_rules! Depcrate_evp_pkeyimpl_713 {
() => {
// Module: crate::evp_pkey
// Provides: {"impl_713"}
// Dependencies: {}
impl Clone for LcPtr < EVP_PKEY > { fn clone (& self) -> Self { assert_eq ! (1 , unsafe { EVP_PKEY_up_ref (* self . as_mut_unsafe ()) } , "infallible AWS-LC function") ; Self :: new (unsafe { * self . as_mut_unsafe () }) . expect ("non-null AWS-LC EVP_PKEY pointer") } }
};
}
