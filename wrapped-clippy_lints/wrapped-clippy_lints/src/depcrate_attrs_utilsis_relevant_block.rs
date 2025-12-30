// Generated macro for is_relevant_block (function)
macro_rules! Depcrate_attrs_utilsis_relevant_block {
() => {
// Module: crate::attrs::utils
// Provides: {"is_relevant_block"}
// Dependencies: {}
fn is_relevant_block (cx : & LateContext < '_ > , typeck_results : & ty :: TypeckResults < '_ > , block : & Block < '_ >) -> bool { block . stmts . first () . map_or_else (| | { block . expr . as_ref () . is_some_and (| e | is_relevant_expr (cx , typeck_results , e)) } , | stmt | match & stmt . kind { StmtKind :: Let (_) => true , StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => is_relevant_expr (cx , typeck_results , expr) , StmtKind :: Item (_) => false , } ,) }
};
}
