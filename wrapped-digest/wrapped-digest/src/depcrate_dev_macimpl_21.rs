// Generated macro for impl_21 (impl)
macro_rules! Depcrate_dev_macimpl_21 {
() => {
// Module: crate::dev::mac
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : OutputSizeUser > ConstantTimeEq for CtOutput < T > { # [inline (always)] fn ct_eq (& self , other : & Self) -> Choice { self . bytes . ct_eq (& other . bytes) } }
};
}
