// Generated macro for impl_1637 (impl)
macro_rules! Depcrate_syncimpl_1637 {
() => {
// Module: crate::sync
// Provides: {"impl_1637"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T , I : Iterator < Item = T > > ToArcSlice < T > for I { default fn to_arc_slice (self) -> Arc < [T] > { self . collect :: < Vec < T > > () . into () } }
};
}
