// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_rcimpl_1241 {
() => {
// Module: crate::rc
// Provides: {"impl_1241"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "more_rc_default_impls" , since = "1.80.0")] impl < T > Default for Rc < [T] > { # [doc = " Creates an empty `[T]` inside an `Rc`."] # [doc = ""] # [doc = " This may or may not share an allocation with other Rcs on the same thread."] # [inline] fn default () -> Self { let arr : [T ; 0] = [] ; Rc :: from (arr) } }
};
}
