// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1128 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1128"}
// Dependencies: {}
# [stable (feature = "box_from_c_string" , since = "1.20.0")] impl From < CString > for Box < CStr > { # [doc = " Converts a [`CString`] into a <code>[Box]<[CStr]></code> without copying or allocating."] # [inline] fn from (s : CString) -> Box < CStr > { s . into_boxed_c_str () } }
};
}
