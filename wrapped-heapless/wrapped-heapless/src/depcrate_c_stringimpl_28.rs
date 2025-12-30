// Generated macro for impl_28 (impl)
macro_rules! Depcrate_c_stringimpl_28 {
() => {
// Module: crate::c_string
// Provides: {"impl_28"}
// Dependencies: {}
impl < const N : usize , LenT : LenType > Ord for CString < N , LenT > { # [inline] fn cmp (& self , rhs : & Self) -> Ordering { self . as_c_str () . cmp (rhs . as_c_str ()) } }
};
}
