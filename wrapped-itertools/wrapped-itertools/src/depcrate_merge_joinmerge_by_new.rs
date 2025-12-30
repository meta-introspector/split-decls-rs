// Generated macro for merge_by_new (function)
macro_rules! Depcrate_merge_joinmerge_by_new {
() => {
// Module: crate::merge_join
// Provides: {"merge_by_new"}
// Dependencies: {}
# [doc = " Create a `MergeBy` iterator."] pub fn merge_by_new < I , J , F > (a : I , b : J , cmp : F) -> MergeBy < I :: IntoIter , J :: IntoIter , F > where I : IntoIterator , J : IntoIterator < Item = I :: Item > , { MergeBy { left : put_back (a . into_iter () . fuse ()) , right : put_back (b . into_iter () . fuse ()) , cmp_fn : cmp , } }
};
}
