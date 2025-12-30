// Generated macro for impl_21 (impl)
macro_rules! Depcrate_c_stringimpl_21 {
() => {
// Module: crate::c_string
// Provides: {"impl_21"}
// Dependencies: {}
impl < const N : usize , LenT : LenType > AsRef < CStr > for CString < N , LenT > { # [inline] fn as_ref (& self) -> & CStr { self . as_c_str () } }
};
}
