// Generated macro for impl_1126 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1126 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1126"}
// Dependencies: {}
# [stable (feature = "c_string_from_str" , since = "1.85.0")] impl TryFrom < CString > for String { type Error = IntoStringError ; # [doc = " Converts a [`CString`] into a [`String`] if it contains valid UTF-8 data."] # [doc = ""] # [doc = " This method is equivalent to [`CString::into_string`]."] # [inline] fn try_from (value : CString) -> Result < Self , Self :: Error > { value . into_string () } }
};
}
