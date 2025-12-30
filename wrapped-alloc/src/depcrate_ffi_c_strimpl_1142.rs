// Generated macro for impl_1142 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1142 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1142"}
// Dependencies: {}
# [stable (feature = "cstring_from_vec_with_nul" , since = "1.58.0")] impl fmt :: Display for FromVecWithNulError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . error_kind { FromBytesWithNulErrorKind :: InteriorNul (pos) => { write ! (f , "data provided contains an interior nul byte at pos {pos}") } FromBytesWithNulErrorKind :: NotNulTerminated => { write ! (f , "data provided is not nul terminated") } } } }
};
}
