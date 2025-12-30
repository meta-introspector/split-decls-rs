// Generated macro for impl_1271 (impl)
macro_rules! Depcrate_rcimpl_1271 {
() => {
// Module: crate::rc
// Provides: {"impl_1271"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T , I : iter :: TrustedLen < Item = T > > ToRcSlice < T > for I { fn to_rc_slice (self) -> Rc < [T] > { let (low , high) = self . size_hint () ; if let Some (high) = high { debug_assert_eq ! (low , high , "TrustedLen iterator's size hint is not exact: {:?}" , (low , high)) ; unsafe { Rc :: from_iter_exact (self , low) } } else { panic ! ("capacity overflow") ; } } }
};
}
