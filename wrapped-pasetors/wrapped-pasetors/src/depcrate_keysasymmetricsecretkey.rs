// Generated macro for AsymmetricSecretKey (struct)
macro_rules! Depcrate_keysAsymmetricSecretKey {
() => {
// Module: crate::keys
// Provides: {"AsymmetricSecretKey"}
// Dependencies: {}
# [derive (Clone)] # [doc = " An asymmetric secret key used for `.public` tokens, given a version `V`."] # [doc = ""] # [doc = " In case of Ed25519, which is used in V2 and V4, this is the seed concatenated with the public key."] pub struct AsymmetricSecretKey < V > { pub (crate) bytes : Vec < u8 > , pub (crate) phantom : PhantomData < V > , }
};
}
