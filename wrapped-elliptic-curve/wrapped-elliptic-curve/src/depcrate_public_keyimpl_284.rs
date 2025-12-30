// Generated macro for impl_284 (impl)
macro_rules! Depcrate_public_keyimpl_284 {
() => {
// Module: crate::public_key
// Provides: {"impl_284"}
// Dependencies: {}
impl < C > From < & PublicKey < C > > for NonIdentity < AffinePoint < C > > where C : CurveArithmetic , { fn from (value : & PublicKey < C >) -> Self { PublicKey :: to_nonidentity (value) } }
};
}
