// Generated macro for KeyPair (struct)
macro_rules! Depcrate_x25519KeyPair {
() => {
// Module: crate::x25519
// Provides: {"KeyPair"}
// Dependencies: {}
# [doc = " A key pair."] # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub struct KeyPair { # [doc = " Public key part of the key pair."] pub pk : PublicKey , # [doc = " Secret key part of the key pair."] pub sk : SecretKey , }
};
}
