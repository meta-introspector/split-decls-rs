// Generated macro for try_parse_x25519_public_key_raw_bytes (function)
macro_rules! Depcrate_agreementtry_parse_x25519_public_key_raw_bytes {
() => {
// Module: crate::agreement
// Provides: {"try_parse_x25519_public_key_raw_bytes"}
// Dependencies: {}
fn try_parse_x25519_public_key_raw_bytes (key_bytes : & [u8]) -> Result < LcPtr < EVP_PKEY > , KeyRejected > { let expected_pub_key_len = X25519 . id . pub_key_len () ; if key_bytes . len () != expected_pub_key_len { return Err (KeyRejected :: invalid_encoding ()) ; } LcPtr :: < EVP_PKEY > :: parse_raw_public_key (key_bytes , EVP_PKEY_X25519) }
};
}
