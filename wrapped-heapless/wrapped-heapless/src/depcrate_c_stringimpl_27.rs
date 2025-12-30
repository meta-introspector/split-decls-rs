// Generated macro for impl_27 (impl)
macro_rules! Depcrate_c_stringimpl_27 {
() => {
// Module: crate::c_string
// Provides: {"impl_27"}
// Dependencies: {}
impl < const N : usize , const M : usize , LenT1 : LenType , LenT2 : LenType > PartialOrd < CString < M , LenT2 > > for CString < N , LenT1 > { # [inline] fn partial_cmp (& self , rhs : & CString < M , LenT2 >) -> Option < Ordering > { self . as_c_str () . partial_cmp (rhs . as_c_str ()) } }
};
}
