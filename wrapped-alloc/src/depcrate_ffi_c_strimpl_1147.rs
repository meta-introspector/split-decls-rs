// Generated macro for impl_1147 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1147 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1147"}
// Dependencies: {}
# [stable (feature = "c_string_eq_c_str" , since = "1.90.0")] impl PartialEq < CStr > for CString { # [inline] fn eq (& self , other : & CStr) -> bool { * * self == * other } # [inline] fn ne (& self , other : & CStr) -> bool { * * self != * other } }
};
}
