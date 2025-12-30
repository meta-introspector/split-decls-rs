// Generated macro for impl_1153 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1153 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1153"}
// Dependencies: {}
# [stable (feature = "c_string_eq_c_str" , since = "1.90.0")] impl PartialEq < CString > for CStr { # [inline] fn eq (& self , other : & CString) -> bool { * self == * * other } # [inline] fn ne (& self , other : & CString) -> bool { * self != * * other } }
};
}
