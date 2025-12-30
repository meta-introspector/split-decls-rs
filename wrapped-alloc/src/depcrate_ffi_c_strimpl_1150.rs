// Generated macro for impl_1150 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1150 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1150"}
// Dependencies: {}
# [stable (feature = "cstring_asref" , since = "1.7.0")] impl ops :: Index < ops :: RangeFull > for CString { type Output = CStr ; # [inline] fn index (& self , _index : ops :: RangeFull) -> & CStr { self } }
};
}
