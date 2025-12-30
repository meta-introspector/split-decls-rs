// Generated macro for impl_1155 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1155 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1155"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "c_string_eq_c_str" , since = "1.90.0")] impl PartialEq < CStr > for Cow < '_ , CStr > { # [inline] fn eq (& self , other : & CStr) -> bool { * * self == * other } # [inline] fn ne (& self , other : & CStr) -> bool { * * self != * other } }
};
}
