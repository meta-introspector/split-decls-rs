// Generated macro for impl_489 (impl)
macro_rules! Depcrate_merge_joinimpl_489 {
() => {
// Module: crate::merge_join
// Provides: {"impl_489"}
// Dependencies: {}
impl < I , J , F > Clone for MergeBy < I , J , F > where I : Iterator , J : Iterator , PutBack < Fuse < I > > : Clone , PutBack < Fuse < J > > : Clone , F : Clone , { clone_fields ! (left , right , cmp_fn) ; }
};
}
