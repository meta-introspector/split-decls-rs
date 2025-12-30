// Generated macro for impl_1133 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1133 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1133"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < & CStr > for Arc < CStr > { # [doc = " Converts a `&CStr` into a `Arc<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Arc`]."] # [inline] fn from (s : & CStr) -> Arc < CStr > { let arc : Arc < [u8] > = Arc :: from (s . to_bytes_with_nul ()) ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const CStr) } } }
};
}
