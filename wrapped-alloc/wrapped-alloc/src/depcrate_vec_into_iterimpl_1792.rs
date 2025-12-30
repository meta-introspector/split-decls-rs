// Generated macro for impl_1792 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1792 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1792"}
// Dependencies: {}
# [unstable (issue = "none" , feature = "inplace_iteration")] # [doc (hidden)] unsafe impl < T , A : Allocator > SourceIter for IntoIter < T , A > { type Source = Self ; # [inline] unsafe fn as_inner (& mut self) -> & mut Self :: Source { self } }
};
}
