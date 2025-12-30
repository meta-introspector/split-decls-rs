// Generated macro for AsymmetricKeyPair (struct)
macro_rules! Depcrate_keysAsymmetricKeyPair {
() => {
// Module: crate::keys
// Provides: {"AsymmetricKeyPair"}
// Dependencies: {}
# [derive (Debug , Clone)] # [doc = " A keypair of an [`AsymmetricSecretKey`] and its corresponding [`AsymmetricPublicKey`]."] pub struct AsymmetricKeyPair < V > { # [doc = " The [`AsymmetricSecretKey`]."] pub public : AsymmetricPublicKey < V > , # [doc = " The [`AsymmetricPublicKey`]."] pub secret : AsymmetricSecretKey < V > , }
};
}
