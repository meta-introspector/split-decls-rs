// Generated macro for impl_1638 (impl)
macro_rules! Depcrate_syncimpl_1638 {
() => {
// Module: crate::sync
// Provides: {"impl_1638"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T , I : iter :: TrustedLen < Item = T > > ToArcSlice < T > for I { fn to_arc_slice (self) -> Arc < [T] > { let (low , high) = self . size_hint () ; if let Some (high) = high { debug_assert_eq ! (low , high , "TrustedLen iterator's size hint is not exact: {:?}" , (low , high)) ; unsafe { Arc :: from_iter_exact (self , low) } } else { panic ! ("capacity overflow") ; } } }
};
}
