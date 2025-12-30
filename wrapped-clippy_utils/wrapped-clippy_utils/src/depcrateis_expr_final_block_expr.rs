// Generated macro for is_expr_final_block_expr (function)
macro_rules! Depcrateis_expr_final_block_expr {
() => {
// Module: crate
// Provides: {"is_expr_final_block_expr"}
// Dependencies: {}
# [doc = " Checks if the expression is the final expression returned from a block."] pub fn is_expr_final_block_expr (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> bool { matches ! (tcx . parent_hir_node (expr . hir_id) , Node :: Block (..)) }
};
}
