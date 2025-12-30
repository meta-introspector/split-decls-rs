// Generated macro for impl_498 (impl)
macro_rules! Depcrate_wrappingimpl_498 {
() => {
// Module: crate::wrapping
// Provides: {"impl_498"}
// Dependencies: {}
impl < T : ConstantTimeEq > ConstantTimeEq for Wrapping < T > { # [inline] fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
};
}
