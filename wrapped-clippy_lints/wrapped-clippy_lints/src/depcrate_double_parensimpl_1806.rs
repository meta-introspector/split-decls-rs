// Generated macro for impl_1806 (impl)
macro_rules! Depcrate_double_parensimpl_1806 {
() => {
// Module: crate::double_parens
// Provides: {"impl_1806"}
// Dependencies: {}
impl EarlyLintPass for DoubleParens { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { match & expr . kind { ExprKind :: Paren (inner) if matches ! (inner . kind , ExprKind :: Paren (_) | ExprKind :: Tup (_)) => { if expr . span . eq_ctxt (inner . span) && ! expr . span . in_external_macro (cx . sess () . source_map ()) && check_source (cx , inner) { let mut applicability = Applicability :: MachineApplicable ; let sugg = snippet_with_applicability (cx . sess () , inner . span , "_" , & mut applicability) ; span_lint_and_sugg (cx , DOUBLE_PARENS , expr . span , "unnecessary parentheses" , "remove them" , sugg . to_string () , applicability ,) ; } } , ExprKind :: Call (_ , args) | ExprKind :: MethodCall (box MethodCall { args , .. }) if let [arg] = & * * args && let ExprKind :: Paren (inner) = & arg . kind => { if expr . span . eq_ctxt (arg . span) && ! arg . span . in_external_macro (cx . sess () . source_map ()) && check_source (cx , arg) { let mut applicability = Applicability :: MachineApplicable ; let sugg = snippet_with_context (cx . sess () , inner . span , arg . span . ctxt () , "_" , & mut applicability) . 0 ; span_lint_and_sugg (cx , DOUBLE_PARENS , arg . span , "unnecessary parentheses" , "remove them" , sugg . to_string () , applicability ,) ; } } , _ => { } , } } }
};
}
