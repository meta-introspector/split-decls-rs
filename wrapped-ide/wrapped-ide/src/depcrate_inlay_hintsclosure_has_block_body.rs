// Generated macro for closure_has_block_body (function)
macro_rules! Depcrate_inlay_hintsclosure_has_block_body {
() => {
// Module: crate::inlay_hints
// Provides: {"closure_has_block_body"}
// Dependencies: {}
fn closure_has_block_body (closure : & ast :: ClosureExpr) -> bool { matches ! (closure . body () , Some (ast :: Expr :: BlockExpr (_))) }
};
}
