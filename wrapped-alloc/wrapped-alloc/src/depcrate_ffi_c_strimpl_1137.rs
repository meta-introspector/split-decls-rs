// Generated macro for impl_1137 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1137 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1137"}
// Dependencies: {}
# [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut CStr > for Rc < CStr > { # [doc = " Converts a `&mut CStr` into a `Rc<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Rc`]."] # [inline] fn from (s : & mut CStr) -> Rc < CStr > { Rc :: from (& * s) } }
};
}
