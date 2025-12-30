// Generated macro for is_empty_body (function)
macro_rules! Depcrate_extra_unused_type_parametersis_empty_body {
() => {
// Module: crate::extra_unused_type_parameters
// Provides: {"is_empty_body"}
// Dependencies: {}
fn is_empty_body (cx : & LateContext < '_ > , body : BodyId) -> bool { matches ! (cx . tcx . hir_body (body) . value . kind , ExprKind :: Block (b , _) if b . stmts . is_empty () && b . expr . is_none ()) }
};
}
