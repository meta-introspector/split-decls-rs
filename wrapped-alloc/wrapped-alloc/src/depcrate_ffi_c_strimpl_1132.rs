// Generated macro for impl_1132 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1132 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1132"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < CString > for Arc < CStr > { # [doc = " Converts a [`CString`] into an <code>[Arc]<[CStr]></code> by moving the [`CString`]"] # [doc = " data into a new [`Arc`] buffer."] # [inline] fn from (s : CString) -> Arc < CStr > { let arc : Arc < [u8] > = Arc :: from (s . into_inner ()) ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const CStr) } } }
};
}
