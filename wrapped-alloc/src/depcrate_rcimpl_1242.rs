// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_rcimpl_1242 {
() => {
// Module: crate::rc
// Provides: {"impl_1242"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "pin_default_impls" , since = "1.91.0")] impl < T > Default for Pin < Rc < T > > where T : ? Sized , Rc < T > : Default , { # [inline] fn default () -> Self { unsafe { Pin :: new_unchecked (Rc :: < T > :: default ()) } } }
};
}
