// Generated macro for impl_244 (impl)
macro_rules! Depcrate_secret_keyimpl_244 {
() => {
// Module: crate::secret_key
// Provides: {"impl_244"}
// Dependencies: {}
impl < C > Drop for SecretKey < C > where C : Curve , { fn drop (& mut self) { self . inner . zeroize () ; } }
};
}
