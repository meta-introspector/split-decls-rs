// Generated macro for impl_25 (impl)
macro_rules! Depcrate_nodes_btreeimpl_25 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_25"}
// Dependencies: {}
# [allow (unsafe_code)] impl < A > PoolDefault for Node < A > { # [cfg (feature = "pool")] unsafe fn default_uninit (target : & mut mem :: MaybeUninit < Self >) { let ptr : * mut Self = target . as_mut_ptr () ; Chunk :: default_uninit (cast_uninit (& mut (* ptr) . keys)) ; Chunk :: default_uninit (cast_uninit (& mut (* ptr) . children)) ; (* ptr) . children . push_back (None) ; } }
};
}
