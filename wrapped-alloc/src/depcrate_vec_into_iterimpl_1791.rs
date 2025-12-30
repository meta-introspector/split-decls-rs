// Generated macro for impl_1791 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1791 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1791"}
// Dependencies: {}
# [unstable (issue = "none" , feature = "inplace_iteration")] # [doc (hidden)] unsafe impl < T , A : Allocator > InPlaceIterable for IntoIter < T , A > { const EXPAND_BY : Option < NonZero < usize > > = NonZero :: new (1) ; const MERGE_BY : Option < NonZero < usize > > = NonZero :: new (1) ; }
};
}
