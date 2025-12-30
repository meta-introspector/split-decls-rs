// Generated macro for impl_64 (impl)
macro_rules! Depcrate_default_expressionimpl_64 {
() => {
// Module: crate::default_expression
// Provides: {"impl_64"}
// Dependencies: {}
impl darling :: FromMeta for DefaultExpression { fn from_word () -> darling :: Result < Self > { Ok (DefaultExpression :: Trait) } fn from_expr (expr : & syn :: Expr) -> darling :: Result < Self > { if let syn :: Expr :: Lit (el) = expr { if let syn :: Lit :: Str (_) = el . lit { return Self :: from_value (& el . lit) ; } } Ok (Self :: Explicit (expr . clone () . into ())) } fn from_value (value : & syn :: Lit) -> darling :: Result < Self > { Ok (Self :: Explicit (BlockContents :: from_value (value) ?)) } }
};
}
