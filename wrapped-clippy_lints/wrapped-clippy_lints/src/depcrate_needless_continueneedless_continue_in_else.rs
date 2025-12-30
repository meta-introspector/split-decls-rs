// Generated macro for needless_continue_in_else (function)
macro_rules! Depcrate_needless_continueneedless_continue_in_else {
() => {
// Module: crate::needless_continue
// Provides: {"needless_continue_in_else"}
// Dependencies: {}
# [doc = " Given an expression, returns true if either of the following is true"] # [doc = ""] # [doc = " - The expression is a `continue` node."] # [doc = " - The expression node is a block with the first statement being a `continue`."] fn needless_continue_in_else (else_expr : & Expr < '_ > , label : Option < & Label >) -> bool { match else_expr . kind { ExprKind :: Block (else_block , _) => is_first_block_stmt_continue (else_block , label) , ExprKind :: Continue (l) => compare_labels (label , l . label . as_ref ()) , _ => false , } }
};
}
