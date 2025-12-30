// Generated macro for impl_795 (impl)
macro_rules! Depcrate_provider_skeleton_runtimeimpl_795 {
() => {
// Module: crate::provider::skeleton::runtime
// Provides: {"impl_795"}
// Dependencies: {}
impl From < reference :: Skeleton > for Skeleton < '_ > { fn from (input : reference :: Skeleton) -> Self { let fields = input . fields_iter () . copied () . collect :: < Vec < _ > > () ; Self (ZeroVec :: alloc_from_slice (& fields)) } }
};
}
