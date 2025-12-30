// Generated macro for impl_381 (impl)
macro_rules! Depcrate_iter_chainimpl_381 {
() => {
// Module: crate::iter::chain
// Provides: {"impl_381"}
// Dependencies: {}
impl < A , B > ChainSeq < A , B > { fn new (a : A , b : B) -> ChainSeq < A , B > where A : ExactSizeIterator , B : ExactSizeIterator < Item = A :: Item > , { ChainSeq { chain : a . chain (b) } } }
};
}
