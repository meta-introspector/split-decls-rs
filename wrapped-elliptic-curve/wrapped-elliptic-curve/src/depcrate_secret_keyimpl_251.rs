// Generated macro for impl_251 (impl)
macro_rules! Depcrate_secret_keyimpl_251 {
() => {
// Module: crate::secret_key
// Provides: {"impl_251"}
// Dependencies: {}
# [cfg (feature = "arithmetic")] impl < C > From < & NonZeroScalar < C > > for SecretKey < C > where C : CurveArithmetic , { fn from (scalar : & NonZeroScalar < C >) -> SecretKey < C > { SecretKey { inner : scalar . into () , } } }
};
}
