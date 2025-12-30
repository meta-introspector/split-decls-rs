// Generated macro for impl_26 (impl)
macro_rules! Depcrate_nodes_btreeimpl_26 {
() => {
// Module: crate::nodes::btree
// Provides: {"impl_26"}
// Dependencies: {}
# [allow (unsafe_code)] impl < A > PoolClone for Node < A > where A : Clone , { # [cfg (feature = "pool")] unsafe fn clone_uninit (& self , target : & mut mem :: MaybeUninit < Self >) { self . keys . clone_uninit (cast_uninit (& mut (* target . as_mut_ptr ()) . keys)) ; self . children . clone_uninit (cast_uninit (& mut (* target . as_mut_ptr ()) . children)) ; } }
};
}
