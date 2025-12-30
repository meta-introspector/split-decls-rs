// Generated macro for impl_1134 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1134 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1134"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut CStr > for Arc < CStr > { # [doc = " Converts a `&mut CStr` into a `Arc<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Arc`]."] # [inline] fn from (s : & mut CStr) -> Arc < CStr > { Arc :: from (& * s) } }
};
}
