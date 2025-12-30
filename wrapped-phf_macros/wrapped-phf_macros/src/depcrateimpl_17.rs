// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Parse for Key { fn parse (input : ParseStream < '_ >) -> parse :: Result < Key > { let attrs = input . call (syn :: Attribute :: parse_outer) ? ; let expr = input . parse :: < Expr > () ? ; let (exprs , parsed_keys) = extract_keys_from_expr (& expr) ? ; Ok (Key { parsed : parsed_keys , expr : exprs , attrs , }) } }
};
}
