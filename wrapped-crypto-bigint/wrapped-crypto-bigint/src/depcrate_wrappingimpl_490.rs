// Generated macro for impl_490 (impl)
macro_rules! Depcrate_wrappingimpl_490 {
() => {
// Module: crate::wrapping
// Provides: {"impl_490"}
// Dependencies: {}
impl < T : ConstantTimeEq > ConstantTimeEq for Wrapping < T > { # [inline] fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
};
}
