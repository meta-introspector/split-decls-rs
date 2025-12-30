// Generated macro for PrivateKey (struct)
macro_rules! Depcrate_ecdhPrivateKey {
() => {
// Module: crate::ecdh
// Provides: {"PrivateKey"}
// Dependencies: {}
# [doc = " An ECDH private key over the given curve."] pub struct PrivateKey < C : ec :: Curve > { key : ec :: Key , marker : PhantomData < C > , }
};
}
