// Generated macro for impl_3745 (impl)
macro_rules! Depcrate_loops_mut_range_boundimpl_3745 {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"impl_3745"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for BreakAfterExprVisitor { type Result = ControlFlow < () > ; fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> ControlFlow < () > { if expr . hir_id == self . hir_id { self . past_expr = true ; ControlFlow :: Continue (()) } else if self . past_expr { if matches ! (& expr . kind , ExprKind :: Break (..)) { self . break_after_expr = true ; } ControlFlow :: Break (()) } else { intravisit :: walk_expr (self , expr) } } }
};
}
