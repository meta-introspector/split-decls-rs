// Generated macro for impl_1154 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1154 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1154"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "c_string_eq_c_str" , since = "1.90.0")] impl PartialEq < Cow < '_ , Self > > for CStr { # [inline] fn eq (& self , other : & Cow < '_ , Self >) -> bool { * self == * * other } # [inline] fn ne (& self , other : & Cow < '_ , Self >) -> bool { * self != * * other } }
};
}
