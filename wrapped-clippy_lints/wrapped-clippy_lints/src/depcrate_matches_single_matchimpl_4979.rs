// Generated macro for impl_4979 (impl)
macro_rules! Depcrate_matches_single_matchimpl_4979 {
() => {
// Module: crate::matches::single_match
// Provides: {"impl_4979"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for PatVisitor < 'tcx > { type Result = ControlFlow < () > ; fn visit_pat (& mut self , pat : & 'tcx Pat < '_ >) -> Self :: Result { if matches ! (pat . kind , PatKind :: Binding (..)) { ControlFlow :: Break (()) } else { self . has_enum |= self . typeck . pat_ty (pat) . ty_adt_def () . is_some_and (AdtDef :: is_enum) ; walk_pat (self , pat) } } }
};
}
