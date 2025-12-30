// Generated macro for is_first_block_stmt_continue (function)
macro_rules! Depcrate_needless_continueis_first_block_stmt_continue {
() => {
// Module: crate::needless_continue
// Provides: {"is_first_block_stmt_continue"}
// Dependencies: {}
fn is_first_block_stmt_continue (block : & Block < '_ > , label : Option < & Label >) -> bool { block . stmts . first () . is_some_and (| stmt | match stmt . kind { StmtKind :: Semi (e) | StmtKind :: Expr (e) => { if let ExprKind :: Continue (l) = e . kind { compare_labels (label , l . label . as_ref ()) } else { false } } , _ => false , }) }
};
}
