// Generated macro for impl_1148 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1148 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1148"}
// Dependencies: {}
# [stable (feature = "c_string_eq_c_str" , since = "1.90.0")] impl PartialEq < & CStr > for CString { # [inline] fn eq (& self , other : & & CStr) -> bool { * * self == * * other } # [inline] fn ne (& self , other : & & CStr) -> bool { * * self != * * other } }
};
}
