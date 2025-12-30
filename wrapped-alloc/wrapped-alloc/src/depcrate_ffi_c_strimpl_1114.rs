// Generated macro for impl_1114 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1114 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1114"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ops :: Deref for CString { type Target = CStr ; # [inline] fn deref (& self) -> & CStr { self . as_c_str () } }
};
}
