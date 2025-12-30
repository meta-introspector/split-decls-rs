// Generated macro for impl_163 (impl)
macro_rules! Depcrate_ecdhimpl_163 {
() => {
// Module: crate::ecdh
// Provides: {"impl_163"}
// Dependencies: {}
impl < C > From < & EphemeralSecret < C > > for PublicKey < C > where C : CurveArithmetic , { fn from (ephemeral_secret : & EphemeralSecret < C >) -> Self { ephemeral_secret . public_key () } }
};
}
