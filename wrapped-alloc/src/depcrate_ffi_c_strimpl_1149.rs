// Generated macro for impl_1149 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1149 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1149"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "c_string_eq_c_str" , since = "1.90.0")] impl PartialEq < Cow < '_ , CStr > > for CString { # [inline] fn eq (& self , other : & Cow < '_ , CStr >) -> bool { * * self == * * other } # [inline] fn ne (& self , other : & Cow < '_ , CStr >) -> bool { * * self != * * other } }
};
}
