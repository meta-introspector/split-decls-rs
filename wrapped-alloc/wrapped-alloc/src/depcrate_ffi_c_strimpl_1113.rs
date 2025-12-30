// Generated macro for impl_1113 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1113 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1113"}
// Dependencies: {}
# [stable (feature = "cstring_drop" , since = "1.13.0")] # [rustc_insignificant_dtor] impl Drop for CString { # [inline] fn drop (& mut self) { unsafe { * self . inner . get_unchecked_mut (0) = 0 ; } } }
};
}
