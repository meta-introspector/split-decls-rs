// Generated macro for impl_35 (impl)
macro_rules! Depcrate_iteratorimpl_35 {
() => {
// Module: crate::iterator
// Provides: {"impl_35"}
// Dependencies: {}
impl < L , R > ExactSizeIterator for Either < L , R > where L : ExactSizeIterator , R : ExactSizeIterator < Item = L :: Item > , { fn len (& self) -> usize { for_both ! (self , inner => inner . len ()) } }
};
}
