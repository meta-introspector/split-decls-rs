// Generated macro for impl_3474 (impl)
macro_rules! Depcrate_literal_representationimpl_3474 {
() => {
// Module: crate::literal_representation
// Provides: {"impl_3474"}
// Dependencies: {}
impl EarlyLintPass for LiteralDigitGrouping { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: Lit (lit) = expr . kind && ! expr . span . in_external_macro (cx . sess () . source_map ()) { self . check_lit (cx , lit , expr . span) ; } } }
};
}
