// Generated macro for has_conditional_return (function)
macro_rules! Depcrate_unconditional_recursionhas_conditional_return {
() => {
// Module: crate::unconditional_recursion
// Provides: {"has_conditional_return"}
// Dependencies: {}
fn has_conditional_return (body : & Body < '_ > , expr : & Expr < '_ >) -> bool { match get_return_calls_in_body (body) . as_slice () { [] => false , [return_expr] => return_expr . hir_id != expr . hir_id , _ => true , } }
};
}
