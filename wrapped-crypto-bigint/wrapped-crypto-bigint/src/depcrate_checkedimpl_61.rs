// Generated macro for impl_61 (impl)
macro_rules! Depcrate_checkedimpl_61 {
() => {
// Module: crate::checked
// Provides: {"impl_61"}
// Dependencies: {}
impl < T : ConstantTimeEq > ConstantTimeEq for Checked < T > { # [inline] fn ct_eq (& self , rhs : & Self) -> Choice { self . 0 . ct_eq (& rhs . 0) } }
};
}
