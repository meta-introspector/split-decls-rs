// Generated macro for impl_250 (impl)
macro_rules! Depcrate_secret_keyimpl_250 {
() => {
// Module: crate::secret_key
// Provides: {"impl_250"}
// Dependencies: {}
# [cfg (feature = "arithmetic")] impl < C > From < NonZeroScalar < C > > for SecretKey < C > where C : CurveArithmetic , { fn from (scalar : NonZeroScalar < C >) -> SecretKey < C > { SecretKey :: from (& scalar) } }
};
}
