// Generated macro for impl_241 (impl)
macro_rules! Depcrate_secret_keyimpl_241 {
() => {
// Module: crate::secret_key
// Provides: {"impl_241"}
// Dependencies: {}
impl < C > ConstantTimeEq for SecretKey < C > where C : Curve , { fn ct_eq (& self , other : & Self) -> Choice { self . inner . ct_eq (& other . inner) } }
};
}
