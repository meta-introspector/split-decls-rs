// Generated macro for CryptoOps (trait)
macro_rules! Depcrate_opsCryptoOps {
() => {
// Module: crate::ops
// Provides: {"CryptoOps"}
// Dependencies: {}
pub trait CryptoOps { # [doc = " A public key type for this cryptographic backend."] type Key ; # [doc = " An error type for this cryptographic backend."] type Err ; # [doc = " Extra data that's passed around with the certificate."] type CertificateExtra ; # [doc = " Extra data that's accessible alongside the PolicyDefinition."] type PolicyExtra ; # [doc = " Extracts the public key from the given `Certificate` in"] # [doc = " a `Key` format known by the cryptographic backend, or `None`"] # [doc = " if the key is malformed."] fn public_key (& self , cert : & Certificate < '_ >) -> Result < Self :: Key , Self :: Err > ; # [doc = " Verifies the signature on `Certificate` using the given"] # [doc = " `Key`."] fn verify_signed_by (& self , cert : & Certificate < '_ > , key : & Self :: Key) -> Result < () , Self :: Err > ; fn clone_public_key (extra : & Self :: Key) -> Self :: Key ; fn clone_extra (extra : & Self :: CertificateExtra) -> Self :: CertificateExtra ; }
};
}
