// Generated macro for AsymmetricPublicKey (struct)
macro_rules! Depcrate_keysAsymmetricPublicKey {
() => {
// Module: crate::keys
// Provides: {"AsymmetricPublicKey"}
// Dependencies: {}
# [derive (Debug , Clone)] # [doc = " An asymmetric public key used for `.public` tokens, given a version `V`."] pub struct AsymmetricPublicKey < V > { pub (crate) bytes : Vec < u8 > , pub (crate) phantom : PhantomData < V > , }
};
}
