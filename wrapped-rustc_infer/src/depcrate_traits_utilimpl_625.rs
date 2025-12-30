// Generated macro for impl_625 (impl)
macro_rules! Depcrate_traits_utilimpl_625 {
() => {
// Module: crate::traits::util
// Provides: {"impl_625"}
// Dependencies: {}
impl < 'tcx > Extend < ty :: Predicate < 'tcx > > for PredicateSet < 'tcx > { fn extend < I : IntoIterator < Item = ty :: Predicate < 'tcx > > > (& mut self , iter : I) { for pred in iter { self . insert (pred) ; } } fn extend_one (& mut self , pred : ty :: Predicate < 'tcx >) { self . insert (pred) ; } fn extend_reserve (& mut self , additional : usize) { Extend :: < ty :: Predicate < 'tcx > > :: extend_reserve (& mut self . set , additional) ; } }
};
}
