// Generated macro for new_decrypt_key (function)
macro_rules! Depcrate_aesnew_decrypt_key {
() => {
// Module: crate::aes
// Provides: {"new_decrypt_key"}
// Dependencies: {}
# [doc = " This should only be publicly exposed by wrapper types with the correct key lengths."] # [allow (clippy :: unwrap_used)] fn new_decrypt_key (key : & [u8]) -> DecryptKey { DecryptKey (unsafe { initialized_struct_fallible (| aes_key | { bssl_sys :: AES_set_decrypt_key (key . as_ffi_ptr () , key . len () as c_uint * 8 , aes_key) == 0 }) } . unwrap () ,) }
};
}
