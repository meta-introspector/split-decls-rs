// Generated macro for impl_4910 (impl)
macro_rules! Depcrate_matches_significant_drop_in_scrutineeimpl_4910 {
() => {
// Module: crate::matches::significant_drop_in_scrutinee
// Provides: {"impl_4910"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ArmSigDropHelper < '_ , 'tcx > { fn visit_expr (& mut self , ex : & 'tcx Expr < 'tcx >) { if self . sig_drop_checker . is_sig_drop_expr (ex) { self . found_sig_drop_spans . insert (ex . span) ; return ; } walk_expr (self , ex) ; } }
};
}
