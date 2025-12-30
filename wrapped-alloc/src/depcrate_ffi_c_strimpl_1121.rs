// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1121 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1121"}
// Dependencies: {}
# [stable (feature = "box_from_mut_slice" , since = "1.84.0")] impl From < & mut CStr > for Box < CStr > { # [doc = " Converts a `&mut CStr` into a `Box<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Box`]."] fn from (s : & mut CStr) -> Box < CStr > { Self :: from (& * s) } }
};
}
