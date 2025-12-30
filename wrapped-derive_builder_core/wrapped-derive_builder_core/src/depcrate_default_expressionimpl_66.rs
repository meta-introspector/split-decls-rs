// Generated macro for impl_66 (impl)
macro_rules! Depcrate_default_expressionimpl_66 {
() => {
// Module: crate::default_expression
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > ToTokens for DefaultExpressionWithCrateRoot < 'a > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let crate_root = self . crate_root ; match self . expr { DefaultExpression :: Explicit (ref block) => block . to_tokens (tokens) , DefaultExpression :: Trait => quote ! (# crate_root :: export :: core :: default :: Default :: default ()) . to_tokens (tokens) , } } }
};
}
