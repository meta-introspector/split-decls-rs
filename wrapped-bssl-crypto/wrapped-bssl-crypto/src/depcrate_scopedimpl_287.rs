// Generated macro for impl_287 (impl)
macro_rules! Depcrate_scopedimpl_287 {
() => {
// Module: crate::scoped
// Provides: {"impl_287"}
// Dependencies: {}
impl EvpPkey { pub fn new () -> Self { let ptr = unsafe { bssl_sys :: EVP_PKEY_new () } ; assert ! (! ptr . is_null ()) ; EvpPkey (ptr) } pub fn from_ptr (ptr : * mut bssl_sys :: EVP_PKEY) -> Self { EvpPkey (ptr) } pub fn from_ptr_or_null (ptr : * mut bssl_sys :: EVP_PKEY) -> Option < Self > { if ptr . is_null () { None } else { Some (EvpPkey :: from_ptr (ptr)) } } pub fn from_der_subject_public_key_info (spki : & [u8] , algs : & [* const bssl_sys :: EVP_PKEY_ALG] ,) -> Option < Self > { EvpPkey :: from_ptr_or_null (unsafe { bssl_sys :: EVP_PKEY_from_subject_public_key_info (spki . as_ffi_ptr () , spki . len () , algs . as_ffi_ptr () , algs . len () ,) } ,) } pub fn from_der_private_key_info (spki : & [u8] , algs : & [* const bssl_sys :: EVP_PKEY_ALG] ,) -> Option < Self > { EvpPkey :: from_ptr_or_null (unsafe { bssl_sys :: EVP_PKEY_from_private_key_info (spki . as_ffi_ptr () , spki . len () , algs . as_ffi_ptr () , algs . len () ,) } ,) } pub fn as_ffi_ptr (& mut self) -> * mut bssl_sys :: EVP_PKEY { self . 0 } }
};
}
