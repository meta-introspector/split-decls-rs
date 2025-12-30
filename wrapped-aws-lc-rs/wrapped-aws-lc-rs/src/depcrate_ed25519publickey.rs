// Generated macro for PublicKey (struct)
macro_rules! Depcrate_ed25519PublicKey {
() => {
// Module: crate::ed25519
// Provides: {"PublicKey"}
// Dependencies: {}
# [derive (Clone)] # [allow (clippy :: module_name_repetitions)] # [doc = " Ed25519 Public Key"] pub struct PublicKey { evp_pkey : LcPtr < EVP_PKEY > , public_key_bytes : [u8 ; ED25519_PUBLIC_KEY_LEN] , }
};
}
