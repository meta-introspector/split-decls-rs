// Generated macro for impl_246 (impl)
macro_rules! Depcrate_secret_keyimpl_246 {
() => {
// Module: crate::secret_key
// Provides: {"impl_246"}
// Dependencies: {}
impl < C > PartialEq for SecretKey < C > where C : Curve , { fn eq (& self , other : & Self) -> bool { self . ct_eq (other) . into () } }
};
}
