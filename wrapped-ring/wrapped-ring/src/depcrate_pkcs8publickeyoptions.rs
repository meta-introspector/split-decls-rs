// Generated macro for PublicKeyOptions (struct)
macro_rules! Depcrate_pkcs8PublicKeyOptions {
() => {
// Module: crate::pkcs8
// Provides: {"PublicKeyOptions"}
// Dependencies: {}
pub (crate) struct PublicKeyOptions { # [doc = " Should the wrong public key ASN.1 tagging used by early implementations"] # [doc = " of PKCS#8 v2 (including earlier versions of *ring*) be accepted?"] pub accept_legacy_ed25519_public_key_tag : bool , }
};
}
