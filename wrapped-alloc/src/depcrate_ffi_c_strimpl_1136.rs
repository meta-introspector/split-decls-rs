// Generated macro for impl_1136 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1136 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1136"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < & CStr > for Rc < CStr > { # [doc = " Converts a `&CStr` into a `Rc<CStr>`,"] # [doc = " by copying the contents into a newly allocated [`Rc`]."] # [inline] fn from (s : & CStr) -> Rc < CStr > { let rc : Rc < [u8] > = Rc :: from (s . to_bytes_with_nul ()) ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const CStr) } } }
};
}
