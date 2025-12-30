// Generated macro for parse_ed25519_public_key (function)
macro_rules! Depcrate_ed25519parse_ed25519_public_key {
() => {
// Module: crate::ed25519
// Provides: {"parse_ed25519_public_key"}
// Dependencies: {}
pub (crate) fn parse_ed25519_public_key (key_bytes : & [u8]) -> Result < LcPtr < EVP_PKEY > , KeyRejected > { if key_bytes . len () == ED25519_PUBLIC_KEY_LEN { LcPtr :: < EVP_PKEY > :: parse_raw_public_key (key_bytes , EVP_PKEY_ED25519) } else { LcPtr :: < EVP_PKEY > :: parse_rfc5280_public_key (key_bytes , EVP_PKEY_ED25519) } }
};
}
