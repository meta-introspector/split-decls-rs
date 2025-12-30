// Generated macro for impl_1117 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1117 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1117"}
// Dependencies: {}
# [stable (feature = "cstr_default" , since = "1.10.0")] impl Default for CString { # [doc = " Creates an empty `CString`."] fn default () -> CString { let a : & CStr = Default :: default () ; a . to_owned () } }
};
}
