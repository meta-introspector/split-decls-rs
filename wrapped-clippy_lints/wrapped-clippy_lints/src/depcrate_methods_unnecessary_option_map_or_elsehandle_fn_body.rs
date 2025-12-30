// Generated macro for handle_fn_body (function)
macro_rules! Depcrate_methods_unnecessary_option_map_or_elsehandle_fn_body {
() => {
// Module: crate::methods::unnecessary_option_map_or_else
// Provides: {"handle_fn_body"}
// Dependencies: {}
fn handle_fn_body (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , def_arg : & Expr < '_ > , body : & Body < '_ >) { if let Some (first_param) = body . params . first () { let body_expr = peel_blocks (body . value) ; match body_expr . kind { ExprKind :: Path (qpath) => { handle_qpath (cx , expr , recv , def_arg , first_param . pat . hir_id , qpath) ; } , ExprKind :: Block (block , _) => { if let Some (block_expr) = block . expr && let Some (last_chain_binding_id) = get_last_chain_binding_hir_id (first_param . pat . hir_id , block . stmts) && let ExprKind :: Path (qpath) = block_expr . kind { handle_qpath (cx , expr , recv , def_arg , last_chain_binding_id , qpath) ; } } , _ => { } , } } }
};
}
