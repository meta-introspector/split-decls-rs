// Generated macro for impl_1270 (impl)
macro_rules! Depcrate_rcimpl_1270 {
() => {
// Module: crate::rc
// Provides: {"impl_1270"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T , I : Iterator < Item = T > > ToRcSlice < T > for I { default fn to_rc_slice (self) -> Rc < [T] > { self . collect :: < Vec < T > > () . into () } }
};
}
