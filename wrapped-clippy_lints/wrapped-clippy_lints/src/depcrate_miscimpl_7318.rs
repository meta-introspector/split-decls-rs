// Generated macro for impl_7318 (impl)
macro_rules! Depcrate_miscimpl_7318 {
() => {
// Module: crate::misc
// Provides: {"impl_7318"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for LintPass { fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { if let StmtKind :: Semi (expr) = stmt . kind && let ExprKind :: Binary (binop , a , b) = & expr . kind && matches ! (binop . node , BinOpKind :: And | BinOpKind :: Or) && ! stmt . span . from_expansion () && expr . span . eq_ctxt (stmt . span) { span_lint_hir_and_then (cx , SHORT_CIRCUIT_STATEMENT , expr . hir_id , stmt . span , "boolean short circuit operator in statement may be clearer using an explicit test" , | diag | { let mut app = Applicability :: MachineApplicable ; let test = Sugg :: hir_with_context (cx , a , expr . span . ctxt () , "_" , & mut app) ; let test = if binop . node == BinOpKind :: Or { ! test } else { test } ; let then = Sugg :: hir_with_context (cx , b , expr . span . ctxt () , "_" , & mut app) ; diag . span_suggestion (stmt . span , "replace it with" , format ! ("if {test} {{ {then}; }}") , app) ; } ,) ; } } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if expr . span . in_external_macro (cx . sess () . source_map ()) || expr . span . desugaring_kind () . is_some () || in_automatically_derived (cx . tcx , expr . hir_id) { return ; } used_underscore_binding (cx , expr) ; used_underscore_items (cx , expr) ; } }
};
}
