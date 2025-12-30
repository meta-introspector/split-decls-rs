// Generated macro for is_temporary (function)
macro_rules! Depcrate_temporary_assignmentis_temporary {
() => {
// Module: crate::temporary_assignment
// Provides: {"is_temporary"}
// Dependencies: {}
fn is_temporary (expr : & Expr < '_ >) -> bool { matches ! (& expr . kind , ExprKind :: Struct (..) | ExprKind :: Tup (..)) }
};
}
