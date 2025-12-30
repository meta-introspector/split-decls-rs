// Generated macro for is_let_desugar (function)
macro_rules! Depcrate_needless_for_eachis_let_desugar {
() => {
// Module: crate::needless_for_each
// Provides: {"is_let_desugar"}
// Dependencies: {}
# [doc = " Check if the block is a desugared `_ = expr` statement."] fn is_let_desugar (block : & Block < '_ >) -> bool { matches ! (block , Block { stmts : [Stmt { kind : StmtKind :: Let (_) , .. } ,] , .. }) }
};
}
