// Generated macro for impl_108 (impl)
macro_rules! Depcrate_ecimpl_108 {
() => {
// Module: crate::ec
// Provides: {"impl_108"}
// Dependencies: {}
impl Group { fn as_ffi_ptr (self) -> * const bssl_sys :: EC_GROUP { match self { Group :: P256 => unsafe { bssl_sys :: EC_group_p256 () } , Group :: P384 => unsafe { bssl_sys :: EC_group_p384 () } , } } fn as_evp_pkey_alg (self) -> * const bssl_sys :: EVP_PKEY_ALG { match self { Group :: P256 => unsafe { bssl_sys :: EVP_pkey_ec_p256 () } , Group :: P384 => unsafe { bssl_sys :: EVP_pkey_ec_p384 () } , } } }
};
}
