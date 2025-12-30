// Generated macro for check_for_mutation (function)
macro_rules! Depcrate_loops_mut_range_boundcheck_for_mutation {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"check_for_mutation"}
// Dependencies: {}
fn check_for_mutation (cx : & LateContext < '_ > , body : & Expr < '_ > , bound_id_start : Option < HirId > , bound_id_end : Option < HirId > ,) -> (Option < Span > , Option < Span >) { let mut delegate = MutatePairDelegate { cx , hir_id_low : bound_id_start , hir_id_high : bound_id_end , span_low : None , span_high : None , } ; ExprUseVisitor :: for_clippy (cx , body . hir_id . owner . def_id , & mut delegate) . walk_expr (body) . into_ok () ; delegate . mutation_span () }
};
}
