// Generated macro for is_used (function)
macro_rules! Depcrate_miscis_used {
() => {
// Module: crate::misc
// Provides: {"is_used"}
// Dependencies: {}
# [doc = " Heuristic to see if an expression is used. Should be compatible with"] # [doc = " `unused_variables`'s idea"] # [doc = " of what it means for an expression to be \"used\"."] fn is_used (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { get_parent_expr (cx , expr) . is_none_or (| parent | match parent . kind { ExprKind :: Assign (_ , rhs , _) | ExprKind :: AssignOp (_ , _ , rhs) => SpanlessEq :: new (cx) . eq_expr (rhs , expr) , _ => is_used (cx , parent) , }) }
};
}
