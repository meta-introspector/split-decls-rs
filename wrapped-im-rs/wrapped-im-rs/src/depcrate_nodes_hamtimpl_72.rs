// Generated macro for impl_72 (impl)
macro_rules! Depcrate_nodes_hamtimpl_72 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_72"}
// Dependencies: {}
# [allow (unsafe_code)] impl < A > PoolDefault for Node < A > { # [cfg (feature = "pool")] unsafe fn default_uninit (target : & mut mem :: MaybeUninit < Self >) { SparseChunk :: default_uninit (target . as_mut_ptr () . cast :: < mem :: MaybeUninit < SparseChunk < Entry < A > , HashWidth > > > () . as_mut () . unwrap () ,) } }
};
}
