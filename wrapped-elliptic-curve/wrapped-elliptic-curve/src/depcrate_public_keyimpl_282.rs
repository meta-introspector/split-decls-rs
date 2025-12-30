// Generated macro for impl_282 (impl)
macro_rules! Depcrate_public_keyimpl_282 {
() => {
// Module: crate::public_key
// Provides: {"impl_282"}
// Dependencies: {}
impl < C , P > From < & NonIdentity < P > > for PublicKey < C > where C : CurveArithmetic , P : Copy + Into < AffinePoint < C > > , { fn from (value : & NonIdentity < P >) -> Self { Self { point : value . to_point () . into () , } } }
};
}
