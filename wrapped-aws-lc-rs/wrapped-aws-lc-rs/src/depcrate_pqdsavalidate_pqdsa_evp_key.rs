// Generated macro for validate_pqdsa_evp_key (function)
macro_rules! Depcrate_pqdsavalidate_pqdsa_evp_key {
() => {
// Module: crate::pqdsa
// Provides: {"validate_pqdsa_evp_key"}
// Dependencies: {}
pub (crate) fn validate_pqdsa_evp_key (evp_pkey : & LcPtr < EVP_PKEY > , id : & 'static AlgorithmID ,) -> Result < () , KeyRejected > { if evp_pkey . as_const () . key_size_bytes () == id . pub_key_size_bytes () { Ok (()) } else { Err (KeyRejected :: unspecified ()) } }
};
}
