// Generated macro for ToRcSlice (trait)
macro_rules! Depcrate_rcToRcSlice {
() => {
// Module: crate::rc
// Provides: {"ToRcSlice"}
// Dependencies: {}
# [doc = " Specialization trait used for collecting into `Rc<[T]>`."] # [cfg (not (no_global_oom_handling))] trait ToRcSlice < T > : Iterator < Item = T > + Sized { fn to_rc_slice (self) -> Rc < [T] > ; }
};
}
