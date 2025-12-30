// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1146 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1146"}
// Dependencies: {}
# [stable (feature = "cstring_asref" , since = "1.7.0")] impl From < & CStr > for CString { # [doc = " Converts a <code>&[CStr]</code> into a [`CString`]"] # [doc = " by copying the contents into a new allocation."] fn from (s : & CStr) -> CString { s . to_owned () } }
};
}
