// Generated macro for impl_25 (impl)
macro_rules! Depcrate_c_stringimpl_25 {
() => {
// Module: crate::c_string
// Provides: {"impl_25"}
// Dependencies: {}
impl < const N : usize , const M : usize , LenT1 : LenType , LenT2 : LenType > PartialEq < CString < M , LenT2 > > for CString < N , LenT1 > { # [inline] fn eq (& self , rhs : & CString < M , LenT2 >) -> bool { self . as_c_str () == rhs . as_c_str () } }
};
}
