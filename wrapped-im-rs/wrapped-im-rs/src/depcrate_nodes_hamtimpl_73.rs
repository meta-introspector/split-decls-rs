// Generated macro for impl_73 (impl)
macro_rules! Depcrate_nodes_hamtimpl_73 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_73"}
// Dependencies: {}
# [allow (unsafe_code)] impl < A > PoolClone for Node < A > where A : Clone , { # [cfg (feature = "pool")] unsafe fn clone_uninit (& self , target : & mut mem :: MaybeUninit < Self >) { self . data . clone_uninit (target . as_mut_ptr () . cast :: < mem :: MaybeUninit < SparseChunk < Entry < A > , HashWidth > > > () . as_mut () . unwrap () ,) } }
};
}
