// Generated macro for impl_39 (impl)
macro_rules! Depcrate_iteratorimpl_39 {
() => {
// Module: crate::iterator
// Provides: {"impl_39"}
// Dependencies: {}
impl < L , R > ExactSizeIterator for IterEither < L , R > where L : ExactSizeIterator , R : ExactSizeIterator , { fn len (& self) -> usize { for_both ! (self . inner , ref inner => inner . len ()) } }
};
}
