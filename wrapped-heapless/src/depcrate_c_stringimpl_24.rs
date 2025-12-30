// Generated macro for impl_24 (impl)
macro_rules! Depcrate_c_stringimpl_24 {
() => {
// Module: crate::c_string
// Provides: {"impl_24"}
// Dependencies: {}
impl < const N : usize , LenT : LenType > Deref for CString < N , LenT > { type Target = CStr ; # [inline] fn deref (& self) -> & Self :: Target { self . as_c_str () } }
};
}
