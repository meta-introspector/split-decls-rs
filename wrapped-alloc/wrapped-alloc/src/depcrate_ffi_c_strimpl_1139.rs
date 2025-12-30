// Generated macro for impl_1139 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1139 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1139"}
// Dependencies: {}
# [stable (feature = "default_box_extra" , since = "1.17.0")] impl Default for Box < CStr > { fn default () -> Box < CStr > { let boxed : Box < [u8] > = Box :: from ([0]) ; unsafe { Box :: from_raw (Box :: into_raw (boxed) as * mut CStr) } } }
};
}
