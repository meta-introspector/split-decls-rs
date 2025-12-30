// Generated macro for impl_624 (impl)
macro_rules! Depcrate_traits_utilimpl_624 {
() => {
// Module: crate::traits::util
// Provides: {"impl_624"}
// Dependencies: {}
impl < 'tcx > PredicateSet < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { Self { tcx , set : Default :: default () } } # [doc = " Adds a predicate to the set."] # [doc = ""] # [doc = " Returns whether the predicate was newly inserted. That is:"] # [doc = " - If the set did not previously contain this predicate, `true` is returned."] # [doc = " - If the set already contained this predicate, `false` is returned,"] # [doc = "   and the set is not modified: original predicate is not replaced,"] # [doc = "   and the predicate passed as argument is dropped."] pub fn insert (& mut self , pred : ty :: Predicate < 'tcx >) -> bool { self . set . insert (anonymize_predicate (self . tcx , pred)) } }
};
}
