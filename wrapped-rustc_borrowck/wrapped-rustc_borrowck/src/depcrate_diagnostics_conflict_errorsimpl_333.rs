// Generated macro for impl_333 (impl)
macro_rules! Depcrate_diagnostics_conflict_errorsimpl_333 {
() => {
// Module: crate::diagnostics::conflict_errors
// Provides: {"impl_333"}
// Dependencies: {}
impl < 'hir > Visitor < 'hir > for BreakFinder { fn visit_expr (& mut self , ex : & 'hir hir :: Expr < 'hir >) { match ex . kind { hir :: ExprKind :: Break (destination , _) => { self . found_breaks . push ((destination , ex . span)) ; } hir :: ExprKind :: Continue (destination) => { self . found_continues . push ((destination , ex . span)) ; } _ => { } } hir :: intravisit :: walk_expr (self , ex) ; } }
};
}
