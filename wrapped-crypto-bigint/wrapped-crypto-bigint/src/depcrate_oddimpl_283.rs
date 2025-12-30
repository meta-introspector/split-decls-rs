// Generated macro for impl_283 (impl)
macro_rules! Depcrate_oddimpl_283 {
() => {
// Module: crate::odd
// Provides: {"impl_283"}
// Dependencies: {}
impl < T > ConstantTimeEq for Odd < T > where T : ConstantTimeEq + ? Sized , { fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
};
}
