// Generated macro for impl_29 (impl)
macro_rules! Depcrate_c_stringimpl_29 {
() => {
// Module: crate::c_string
// Provides: {"impl_29"}
// Dependencies: {}
impl < const N : usize , LenT : LenType > fmt :: Debug for CString < N , LenT > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_c_str () . fmt (f) } }
};
}
