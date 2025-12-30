// Generated macro for stmt_might_diverge (function)
macro_rules! Depcrate_mixed_read_write_in_expressionstmt_might_diverge {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"stmt_might_diverge"}
// Dependencies: {}
fn stmt_might_diverge (stmt : & Stmt < '_ >) -> bool { ! matches ! (stmt . kind , StmtKind :: Item (..)) }
};
}
