// Generated macro for impl_4746 (impl)
macro_rules! Depcrate_matches_match_str_case_mismatchimpl_4746 {
() => {
// Module: crate::matches::match_str_case_mismatch
// Provides: {"impl_4746"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for MatchExprVisitor < '_ , 'tcx > { type Result = ControlFlow < CaseMethod > ; fn visit_expr (& mut self , ex : & 'tcx Expr < '_ >) -> Self :: Result { if let ExprKind :: MethodCall (segment , receiver , [] , _) = ex . kind { let result = self . case_altered (segment . ident . name , receiver) ; if result . is_break () { return result ; } } walk_expr (self , ex) } }
};
}
