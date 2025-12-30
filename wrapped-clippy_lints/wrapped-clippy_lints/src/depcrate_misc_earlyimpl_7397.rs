// Generated macro for impl_7397 (impl)
macro_rules! Depcrate_misc_earlyimpl_7397 {
() => {
// Module: crate::misc_early
// Provides: {"impl_7397"}
// Dependencies: {}
impl EarlyLintPass for MiscEarlyLints { fn check_generics (& mut self , cx : & EarlyContext < '_ > , generics : & Generics) { for param in & generics . params { builtin_type_shadow :: check (cx , param) ; } } fn check_pat (& mut self , cx : & EarlyContext < '_ > , pat : & Pat) { if pat . span . in_external_macro (cx . sess () . source_map ()) { return ; } unneeded_field_pattern :: check (cx , pat) ; redundant_pattern :: check (cx , pat) ; redundant_at_rest_pattern :: check (cx , pat) ; unneeded_wildcard_pattern :: check (cx , pat) ; } fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { if expr . span . in_external_macro (cx . sess () . source_map ()) { return ; } if let ExprKind :: Lit (lit) = expr . kind { MiscEarlyLints :: check_lit (cx , lit , expr . span) ; } } }
};
}
