// Generated macro for to_der_subject_public_key_info (function)
macro_rules! Depcrate_ecto_der_subject_public_key_info {
() => {
// Module: crate::ec
// Provides: {"to_der_subject_public_key_info"}
// Dependencies: {}
unsafe fn to_der_subject_public_key_info (ec_key : * mut bssl_sys :: EC_KEY) -> Buffer { let mut pkey = scoped :: EvpPkey :: new () ; assert_eq ! (1 , unsafe { bssl_sys :: EVP_PKEY_set1_EC_KEY (pkey . as_ffi_ptr () , ec_key) }) ; cbb_to_buffer (65 , | cbb | unsafe { assert_eq ! (1 , bssl_sys :: EVP_marshal_public_key (cbb , pkey . as_ffi_ptr ())) ; }) }
};
}
