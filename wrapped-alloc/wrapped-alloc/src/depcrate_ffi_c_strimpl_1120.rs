// Generated macro for impl_1120 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1120 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1120"}
// Dependencies: {}
# [stable (feature = "box_from_c_str" , since = "1.17.0")] impl From < & CStr > for Box < CStr > { # [doc = " Converts a `&CStr` into a `Box<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Box`]."] fn from (s : & CStr) -> Box < CStr > { let boxed : Box < [u8] > = Box :: from (s . to_bytes_with_nul ()) ; unsafe { Box :: from_raw (Box :: into_raw (boxed) as * mut CStr) } } }
};
}
