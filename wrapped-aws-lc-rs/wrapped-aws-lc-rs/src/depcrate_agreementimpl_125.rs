// Generated macro for impl_125 (impl)
macro_rules! Depcrate_agreementimpl_125 {
() => {
// Module: crate::agreement
// Provides: {"impl_125"}
// Dependencies: {}
impl ParsedPublicKey { # [allow (non_upper_case_globals)] pub (crate) fn new (bytes : impl AsRef < [u8] > , nid : i32) -> Result < Self , KeyRejected > { let bytes = bytes . as_ref () . to_vec () . into_boxed_slice () ; if bytes . is_empty () { return Err (KeyRejected :: unspecified ()) ; } match nid { NID_X25519 => { let format : ParsedPublicKeyFormat ; let key = if let Ok (evp_pkey) = LcPtr :: < EVP_PKEY > :: parse_rfc5280_public_key (& bytes , EVP_PKEY_X25519) { format = ParsedPublicKeyFormat :: X509 ; evp_pkey } else { format = ParsedPublicKeyFormat :: Raw ; try_parse_x25519_public_key_raw_bytes (& bytes) ? } ; Ok (ParsedPublicKey { format , nid , key , bytes , }) } NID_X9_62_prime256v1 | NID_secp384r1 | NID_secp521r1 => { let format : ParsedPublicKeyFormat ; let key = if let Ok (evp_pkey) = LcPtr :: < EVP_PKEY > :: parse_rfc5280_public_key (& bytes , EVP_PKEY_EC) { validate_ec_evp_key (& evp_pkey . as_const () , nid) ? ; format = ParsedPublicKeyFormat :: X509 ; evp_pkey } else if let Ok (evp_pkey) = parse_sec1_public_point (& bytes , nid) { format = match bytes [0] { 0x02 | 0x03 => ParsedPublicKeyFormat :: Compressed , 0x04 => ParsedPublicKeyFormat :: Uncompressed , 0x06 | 0x07 => ParsedPublicKeyFormat :: Hybrid , _ => ParsedPublicKeyFormat :: Unknown , } ; evp_pkey } else { return Err (KeyRejected :: invalid_encoding ()) ; } ; Ok (ParsedPublicKey { format , nid , key , bytes , }) } _ => Err (KeyRejected :: unspecified ()) , } } }
};
}
