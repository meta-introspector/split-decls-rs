// Generated macro for MAX_ENCAPSULATED_KEY_LEN (const)
macro_rules! Depcrate_hpkeMAX_ENCAPSULATED_KEY_LEN {
() => {
// Module: crate::hpke
// Provides: {"MAX_ENCAPSULATED_KEY_LEN"}
// Dependencies: {}
# [doc = " Maximum length of the encapsulated key for all currently supported KEMs."] const MAX_ENCAPSULATED_KEY_LEN : usize = bssl_sys :: EVP_HPKE_MAX_ENC_LENGTH as usize ;
};
}
