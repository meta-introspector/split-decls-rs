// Generated macro for KeyPair (trait)
macro_rules! Depcrate_signatureKeyPair {
() => {
// Module: crate::signature
// Provides: {"KeyPair"}
// Dependencies: {}
# [doc = " Key pairs for signing messages (private key and public key)."] pub trait KeyPair : core :: fmt :: Debug + Send + Sized + Sync { # [doc = " The type of the public key."] type PublicKey : AsRef < [u8] > + core :: fmt :: Debug + Clone + Send + Sized + Sync ; # [doc = " The public key for the key pair."] fn public_key (& self) -> & Self :: PublicKey ; }
};
}
