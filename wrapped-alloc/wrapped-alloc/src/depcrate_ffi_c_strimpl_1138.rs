// Generated macro for impl_1138 (impl)
macro_rules! Depcrate_ffi_c_strimpl_1138 {
() => {
// Module: crate::ffi::c_str
// Provides: {"impl_1138"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "more_rc_default_impls" , since = "1.80.0")] impl Default for Rc < CStr > { # [doc = " Creates an empty CStr inside an Rc"] # [doc = ""] # [doc = " This may or may not share an allocation with other Rcs on the same thread."] # [inline] fn default () -> Self { let rc = Rc :: < [u8] > :: from (* b"\0") ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const CStr) } } }
};
}
