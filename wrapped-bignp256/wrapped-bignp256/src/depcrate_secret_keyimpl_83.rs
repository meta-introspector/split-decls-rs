// Generated macro for impl_83 (impl)
macro_rules! Depcrate_secret_keyimpl_83 {
() => {
// Module: crate::secret_key
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (feature = "arithmetic")] impl From < & NonZeroScalar > for SecretKey { fn from (scalar : & NonZeroScalar) -> SecretKey { SecretKey { inner : scalar . into () , } } }
};
}
