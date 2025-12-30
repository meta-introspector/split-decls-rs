// Generated macro for impl_381 (impl)
macro_rules! Depcrate_infer_relate_generalizeimpl_381 {
() => {
// Module: crate::infer::relate::generalize
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for MaxUniverse { fn visit_ty (& mut self , t : Ty < 'tcx >) { if let ty :: Placeholder (placeholder) = t . kind () { self . max_universe = self . max_universe . max (placeholder . universe) ; } t . super_visit_with (self) } fn visit_const (& mut self , c : ty :: Const < 'tcx >) { if let ty :: ConstKind :: Placeholder (placeholder) = c . kind () { self . max_universe = self . max_universe . max (placeholder . universe) ; } c . super_visit_with (self) } fn visit_region (& mut self , r : ty :: Region < 'tcx >) { if let ty :: RePlaceholder (placeholder) = r . kind () { self . max_universe = self . max_universe . max (placeholder . universe) ; } } }
};
}
