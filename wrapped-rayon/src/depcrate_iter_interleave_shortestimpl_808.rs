// Generated macro for impl_808 (impl)
macro_rules! Depcrate_iter_interleave_shortestimpl_808 {
() => {
// Module: crate::iter::interleave_shortest
// Provides: {"impl_808"}
// Dependencies: {}
impl < I , J > InterleaveShortest < I , J > where I : IndexedParallelIterator , J : IndexedParallelIterator < Item = I :: Item > , { # [doc = " Creates a new `InterleaveShortest` iterator"] pub (super) fn new (i : I , j : J) -> Self { InterleaveShortest { interleave : if i . len () <= j . len () { let n = i . len () ; i . take (n) . interleave (j . take (n)) } else { let n = j . len () ; i . take (n + 1) . interleave (j . take (n)) } , } } }
};
}
