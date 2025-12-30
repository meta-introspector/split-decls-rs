// Generated macro for impl_252 (impl)
macro_rules! Depcrate_vecimpl_252 {
() => {
// Module: crate::vec
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T > FromIterator < T > for Vec < T > { # [inline (always)] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Vec < T > { let mut vec = Vec :: new () ; vec . extend (iter) ; vec } }
};
}
