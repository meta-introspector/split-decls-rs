// Generated macro for parse_pqdsa_public_key (function)
macro_rules! Depcrate_pqdsaparse_pqdsa_public_key {
() => {
// Module: crate::pqdsa
// Provides: {"parse_pqdsa_public_key"}
// Dependencies: {}
pub (crate) fn parse_pqdsa_public_key (key_bytes : & [u8] , id : & 'static AlgorithmID ,) -> Result < LcPtr < EVP_PKEY > , KeyRejected > { LcPtr :: < EVP_PKEY > :: parse_rfc5280_public_key (key_bytes , EVP_PKEY_PQDSA) . or (LcPtr :: < EVP_PKEY > :: parse_raw_public_key (key_bytes , EVP_PKEY_PQDSA ,)) . and_then (| key | validate_pqdsa_evp_key (& key , id) . map (| () | key)) }
};
}
