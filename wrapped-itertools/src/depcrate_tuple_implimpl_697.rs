// Generated macro for impl_697 (impl)
macro_rules! Depcrate_tuple_implimpl_697 {
() => {
// Module: crate::tuple_impl
// Provides: {"impl_697"}
// Dependencies: {}
impl < I , T > FusedIterator for TupleWindows < I , T > where I : FusedIterator < Item = T :: Item > , T : HomogeneousTuple + Clone , T :: Item : Clone , { }
};
}
