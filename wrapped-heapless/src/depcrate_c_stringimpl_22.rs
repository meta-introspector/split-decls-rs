// Generated macro for impl_22 (impl)
macro_rules! Depcrate_c_stringimpl_22 {
() => {
// Module: crate::c_string
// Provides: {"impl_22"}
// Dependencies: {}
impl < const N : usize , LenT : LenType > Borrow < CStr > for CString < N , LenT > { # [inline] fn borrow (& self) -> & CStr { self . as_c_str () } }
};
}
