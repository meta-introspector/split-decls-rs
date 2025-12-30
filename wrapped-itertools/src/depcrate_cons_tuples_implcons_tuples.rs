// Generated macro for cons_tuples (function)
macro_rules! Depcrate_cons_tuples_implcons_tuples {
() => {
// Module: crate::cons_tuples_impl
// Provides: {"cons_tuples"}
// Dependencies: {}
# [doc = " Create an iterator that maps for example iterators of"] # [doc = " `((A, B), C)` to `(A, B, C)`."] pub fn cons_tuples < I > (iterable : I) -> ConsTuples < I :: IntoIter > where I : IntoIterator , { ConsTuples { iter : iterable . into_iter () , f : ConsTuplesFn , } }
};
}
