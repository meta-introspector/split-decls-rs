// Generated macro for if_statement_binops (function)
macro_rules! Depcrate_suspicious_operation_groupingsif_statement_binops {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"if_statement_binops"}
// Dependencies: {}
fn if_statement_binops (kind : & ExprKind) -> Option < Vec < BinaryOp < '_ > > > { match kind { ExprKind :: If (condition , _ , _) => chained_binops (& condition . kind) , ExprKind :: Paren (e) => if_statement_binops (& e . kind) , ExprKind :: Block (block , _) => { let mut output = None ; for stmt in & block . stmts { match & stmt . kind { StmtKind :: Expr (e) | StmtKind :: Semi (e) => { output = append_opt_vecs (output , if_statement_binops (& e . kind)) ; } , _ => { } , } } output } , _ => None , } }
};
}
