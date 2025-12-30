// Generated macro for make_stmts_default (macro)
macro_rules! Depcrate_mac_resultmake_stmts_default {
() => {
// Module: crate::mac_result
// Provides: {"make_stmts_default"}
// Dependencies: {}
macro_rules ! make_stmts_default { ($ me : expr , $ TypeParam : ty) => { < dyn MacResult <$ TypeParam >>:: make_expr ($ me) . map (| e | { smallvec ! [ast :: Stmt { id : ast :: DUMMY_NODE_ID , span : e . span , kind : StmtKind :: Expr (e) , }] }) } ; }
};
}
