// Generated macro for impl_2360 (impl)
macro_rules! Depcrate_formattingimpl_2360 {
() => {
// Module: crate::formatting
// Provides: {"impl_2360"}
// Dependencies: {}
impl EarlyLintPass for Formatting { fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & Block) { for w in block . stmts . windows (2) { if let (StmtKind :: Expr (first) , StmtKind :: Expr (second) | StmtKind :: Semi (second)) = (& w [0] . kind , & w [1] . kind) { check_missing_else (cx , first , second) ; } } } fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { check_assign (cx , expr) ; check_unop (cx , expr) ; check_else (cx , expr) ; check_array (cx , expr) ; } }
};
}
