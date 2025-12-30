// Generated macro for expr_needs_parens (function)
macro_rules! Depcrate_hirexpr_needs_parens {
() => {
// Module: crate::hir
// Provides: {"expr_needs_parens"}
// Dependencies: {}
# [doc = " Checks if the specified expression needs parentheses for prefix"] # [doc = " or postfix suggestions to be valid."] # [doc = " For example, `a + b` requires parentheses to suggest `&(a + b)`,"] # [doc = " but just `a` does not."] # [doc = " Similarly, `(a + b).c()` also requires parentheses."] # [doc = " This should not be used for other types of suggestions."] pub fn expr_needs_parens (expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Cast (_ , _) | ExprKind :: Binary (_ , _ , _) => true , _ if is_range_literal (expr) => true , _ => false , } }
};
}
