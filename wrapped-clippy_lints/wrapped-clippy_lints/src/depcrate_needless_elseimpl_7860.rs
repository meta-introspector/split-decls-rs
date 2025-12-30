// Generated macro for impl_7860 (impl)
macro_rules! Depcrate_needless_elseimpl_7860 {
() => {
// Module: crate::needless_else
// Provides: {"impl_7860"}
// Dependencies: {}
impl EarlyLintPass for NeedlessElse { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: If (_ , then_block , Some (else_clause)) = & expr . kind && let ExprKind :: Block (block , _) = & else_clause . kind && ! expr . span . from_expansion () && ! else_clause . span . from_expansion () && block . stmts . is_empty () && let range = (then_block . span . hi () .. expr . span . hi ()) . trim_start (cx) && range . clone () . check_source_text (cx , | src | { ! src . contains (['/' , '#']) }) { span_lint_and_sugg (cx , NEEDLESS_ELSE , range . with_ctxt (expr . span . ctxt ()) , "this `else` branch is empty" , "you can remove it" , String :: new () , Applicability :: MachineApplicable ,) ; } } }
};
}
