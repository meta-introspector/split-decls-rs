// Generated macro for reduce_unit_expression (function)
macro_rules! Depcrate_methods_option_map_or_nonereduce_unit_expression {
() => {
// Module: crate::methods::option_map_or_none
// Provides: {"reduce_unit_expression"}
// Dependencies: {}
fn reduce_unit_expression < 'a > (expr : & 'a hir :: Expr < '_ >) -> Option < (& 'a hir :: Expr < 'a > , & 'a [hir :: Expr < 'a >]) > { match expr . kind { hir :: ExprKind :: Call (func , arg_char) => Some ((func , arg_char)) , hir :: ExprKind :: Block (block , _) => { match (block . stmts , block . expr) { (& [] , Some (inner_expr)) => { reduce_unit_expression (inner_expr) } , _ => None , } } , _ => None , } }
};
}
