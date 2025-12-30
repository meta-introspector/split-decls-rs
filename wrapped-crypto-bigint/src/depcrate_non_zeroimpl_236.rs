// Generated macro for impl_236 (impl)
macro_rules! Depcrate_non_zeroimpl_236 {
() => {
// Module: crate::non_zero
// Provides: {"impl_236"}
// Dependencies: {}
impl < T > ConstantTimeEq for NonZero < T > where T : ConstantTimeEq + ? Sized , { fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
};
}
