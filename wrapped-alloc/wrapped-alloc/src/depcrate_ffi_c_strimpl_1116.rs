// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1116 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1116"}
// Dependencies: {}
# [stable (feature = "cstring_into" , since = "1.7.0")] impl From < CString > for Vec < u8 > { # [doc = " Converts a [`CString`] into a <code>[Vec]<[u8]></code>."] # [doc = ""] # [doc = " The conversion consumes the [`CString`], and removes the terminating NUL byte."] # [inline] fn from (s : CString) -> Vec < u8 > { s . into_bytes () } }
};
}
