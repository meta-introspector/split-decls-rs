// Generated macro for LintData (struct)
macro_rules! Depcrate_needless_continueLintData {
() => {
// Module: crate::needless_continue
// Provides: {"LintData"}
// Dependencies: {}
# [doc = " Data we pass around for construction of help messages."] # [derive (Debug)] struct LintData < 'hir > { # [doc = " The `if` expression encountered in the above loop."] if_expr : & 'hir Expr < 'hir > , # [doc = " The condition expression for the above `if`."] if_cond : & 'hir Expr < 'hir > , # [doc = " The `then` block of the `if` statement."] if_block : & 'hir Block < 'hir > , # [doc = " The `else` block of the `if` statement."] # [doc = " Note that we only work with `if` exprs that have an `else` branch."] else_expr : & 'hir Expr < 'hir > , # [doc = " The 0-based index of the `if` statement in the containing loop block."] stmt_idx : Option < usize > , # [doc = " The statements of the loop block."] loop_block : & 'hir Block < 'hir > , }
};
}
