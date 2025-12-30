// Generated macro for product_handle_default_params (function)
macro_rules! Depcrate_deriveproduct_handle_default_params {
() => {
// Module: crate::derive
// Provides: {"product_handle_default_params"}
// Dependencies: {}
# [doc = " Determine strategy using \"Default\" semantics for a product."] fn product_handle_default_params (ut : & mut UseTracker , ty : Type , span : Span , strategy : StratMode ,) -> StratPair { match strategy { StratMode :: Strategy (strat) => pair_existential (ty , strat) , StratMode :: Value (value) => pair_value (ty , value) , StratMode :: Regex (regex) => pair_regex (ty , regex) , StratMode :: Arbitrary => { ty . mark_uses (ut) ; pair_any (ty , span) } } }
};
}
