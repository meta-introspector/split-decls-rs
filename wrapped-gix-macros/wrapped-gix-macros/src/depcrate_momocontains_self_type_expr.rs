// Generated macro for contains_self_type_expr (function)
macro_rules! Depcrate_momocontains_self_type_expr {
() => {
// Module: crate::momo
// Provides: {"contains_self_type_expr"}
// Dependencies: {}
fn contains_self_type_expr (expr : & Expr) -> bool { match expr { Expr :: Path (ExprPath { qself : Some (_) , .. }) => true , Expr :: Path (ExprPath { path , .. }) => contains_self_type_path (path) , _ => false , } }
};
}
