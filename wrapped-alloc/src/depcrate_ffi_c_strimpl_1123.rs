// Generated macro for impl_1123 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1123 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1123"}
// Dependencies: {}
# [stable (feature = "c_string_from_box" , since = "1.18.0")] impl From < Box < CStr > > for CString { # [doc = " Converts a <code>[Box]<[CStr]></code> into a [`CString`] without copying or allocating."] # [inline] fn from (s : Box < CStr >) -> CString { let raw = Box :: into_raw (s) as * mut [u8] ; CString { inner : unsafe { Box :: from_raw (raw) } } } }
};
}
