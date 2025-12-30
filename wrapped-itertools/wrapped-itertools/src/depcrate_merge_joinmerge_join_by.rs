// Generated macro for merge_join_by (function)
macro_rules! Depcrate_merge_joinmerge_join_by {
() => {
// Module: crate::merge_join
// Provides: {"merge_join_by"}
// Dependencies: {}
# [doc = " Return an iterator adaptor that merge-joins items from the two base iterators in ascending order."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::merge_join_by`]."] pub fn merge_join_by < I , J , F , T > (left : I , right : J , cmp_fn : F ,) -> MergeJoinBy < I :: IntoIter , J :: IntoIter , F > where I : IntoIterator , J : IntoIterator , F : FnMut (& I :: Item , & J :: Item) -> T , { MergeBy { left : put_back (left . into_iter () . fuse ()) , right : put_back (right . into_iter () . fuse ()) , cmp_fn : MergeFuncLR (cmp_fn , PhantomData) , } }
};
}
