// Generated macro for impl_1145 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1145 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1145"}
// Dependencies: {}
# [stable (feature = "cstr_borrow" , since = "1.3.0")] impl ToOwned for CStr { type Owned = CString ; fn to_owned (& self) -> CString { CString { inner : self . to_bytes_with_nul () . into () } } fn clone_into (& self , target : & mut CString) { let mut b = mem :: take (& mut target . inner) . into_vec () ; self . to_bytes_with_nul () . clone_into (& mut b) ; target . inner = b . into_boxed_slice () ; } }
};
}
