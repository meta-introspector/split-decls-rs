// Generated macro for impl_3479 (impl)
macro_rules! Depcrate_literal_representationimpl_3479 {
() => {
// Module: crate::literal_representation
// Provides: {"impl_3479"}
// Dependencies: {}
impl EarlyLintPass for DecimalLiteralRepresentation { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: Lit (lit) = expr . kind && ! expr . span . in_external_macro (cx . sess () . source_map ()) { self . check_lit (cx , lit , expr . span) ; } } }
};
}
