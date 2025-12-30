// Generated macro for impl_97 (impl)
macro_rules! Depcrate_boxedimpl_97 {
() => {
// Module: crate::boxed
// Provides: {"impl_97"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < I > FromIterator < I > for Box < [I] > { # [inline (always)] fn from_iter < T : IntoIterator < Item = I > > (iter : T) -> Self { iter . into_iter () . collect :: < Vec < _ > > () . into_boxed_slice () } }
};
}
