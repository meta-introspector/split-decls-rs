// Generated macro for PublicKey (struct)
macro_rules! Depcrate_ecdsaPublicKey {
() => {
// Module: crate::ecdsa
// Provides: {"PublicKey"}
// Dependencies: {}
# [doc = " An ECDSA public key over the given curve."] pub struct PublicKey < C : ec :: Curve > { point : ec :: Point , marker : PhantomData < C > , }
};
}
