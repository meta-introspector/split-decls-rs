// Generated macro for impl_271 (impl)
macro_rules! Depcrate_vecimpl_271 {
() => {
// Module: crate::vec
// Provides: {"impl_271"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T , const N : usize > From < [T ; N] > for Vec < T > { # [inline (always)] fn from (s : [T ; N]) -> Vec < T > { Box :: slice (Box :: new (s)) . into_vec () } }
};
}
