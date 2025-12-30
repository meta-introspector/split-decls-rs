// Generated macro for with_if_expr (function)
macro_rules! Depcrate_needless_continuewith_if_expr {
() => {
// Module: crate::needless_continue
// Provides: {"with_if_expr"}
// Dependencies: {}
# [doc = " If `stmt` is an if expression node with an `else` branch, calls func with"] # [doc = " the"] # [doc = " following:"] # [doc = ""] # [doc = " - The `if` expression itself,"] # [doc = " - The `if` condition expression,"] # [doc = " - The `then` block, and"] # [doc = " - The `else` expression."] fn with_if_expr < F > (expr : & Expr < '_ > , mut func : F) where F : FnMut (& Expr < '_ > , & Expr < '_ > , & Block < '_ > , & Expr < '_ >) , { if let Some (higher :: If { cond , then , r#else : Some (r#else) , }) = higher :: If :: hir (expr) && let ExprKind :: Block (then , _) = then . kind { func (expr , cond , then , r#else) ; } }
};
}
