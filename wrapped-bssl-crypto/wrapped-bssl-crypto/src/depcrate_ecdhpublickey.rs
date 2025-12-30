// Generated macro for PublicKey (struct)
macro_rules! Depcrate_ecdhPublicKey {
() => {
// Module: crate::ecdh
// Provides: {"PublicKey"}
// Dependencies: {}
# [doc = " An ECDH public key over the given curve."] pub struct PublicKey < C : ec :: Curve > { point : ec :: Point , marker : PhantomData < C > , }
};
}
