// Generated macro for impl_1848 (impl)
macro_rules! Depcrate_else_if_without_elseimpl_1848 {
() => {
// Module: crate::else_if_without_else
// Provides: {"impl_1848"}
// Dependencies: {}
impl EarlyLintPass for ElseIfWithoutElse { fn check_expr (& mut self , cx : & EarlyContext < '_ > , item : & Expr) { if let ExprKind :: If (_ , _ , Some (ref els)) = item . kind && let ExprKind :: If (_ , _ , None) = els . kind && ! item . span . in_external_macro (cx . sess () . source_map ()) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , ELSE_IF_WITHOUT_ELSE , els . span , "`if` expression with an `else if`, but without a final `else`" , | diag | { diag . help ("add an `else` block here") ; } ,) ; } } }
};
}
