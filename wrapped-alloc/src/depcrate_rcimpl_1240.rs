// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_rcimpl_1240 {
() => {
// Module: crate::rc
// Provides: {"impl_1240"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "more_rc_default_impls" , since = "1.80.0")] impl Default for Rc < str > { # [doc = " Creates an empty `str` inside an `Rc`."] # [doc = ""] # [doc = " This may or may not share an allocation with other Rcs on the same thread."] # [inline] fn default () -> Self { let rc = Rc :: < [u8] > :: default () ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const str) } } }
};
}
