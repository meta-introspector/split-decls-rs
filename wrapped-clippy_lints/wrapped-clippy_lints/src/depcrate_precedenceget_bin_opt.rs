// Generated macro for get_bin_opt (function)
macro_rules! Depcrate_precedenceget_bin_opt {
() => {
// Module: crate::precedence
// Provides: {"get_bin_opt"}
// Dependencies: {}
fn get_bin_opt (expr : & Expr) -> Option < BinOpKind > { match expr . kind { ExprKind :: Binary (Spanned { node : op , .. } , _ , _) => Some (op) , _ => None , } }
};
}
