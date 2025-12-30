// Generated macro for new_encrypt_key (function)
macro_rules! Depcrate_aesnew_encrypt_key {
() => {
// Module: crate::aes
// Provides: {"new_encrypt_key"}
// Dependencies: {}
# [doc = " This should only be publicly exposed by wrapper types with the correct key lengths"] # [allow (clippy :: unwrap_used)] fn new_encrypt_key (key : & [u8]) -> EncryptKey { EncryptKey (unsafe { initialized_struct_fallible (| aes_key | { bssl_sys :: AES_set_encrypt_key (key . as_ffi_ptr () , key . len () as c_uint * 8 , aes_key) == 0 }) } . unwrap () ,) }
};
}
