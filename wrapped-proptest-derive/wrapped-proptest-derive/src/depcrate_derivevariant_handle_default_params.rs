// Generated macro for variant_handle_default_params (function)
macro_rules! Depcrate_derivevariant_handle_default_params {
() => {
// Module: crate::derive
// Provides: {"variant_handle_default_params"}
// Dependencies: {}
# [doc = " Determine strategy using \"Default\" semantics for a variant."] fn variant_handle_default_params (ctx : Ctx , ut : & mut UseTracker , v_path : Path , attrs : ParsedAttributes , fields : Vec < Field > ,) -> DeriveResult < StratPair > { let pair = match attrs . strategy { StratMode :: Strategy (strat) => { deny_all_attrs_on_fields (ctx , fields) ? ; pair_existential_self (strat) } StratMode :: Value (value) => { deny_all_attrs_on_fields (ctx , fields) ? ; pair_value_self (value) } StratMode :: Regex (regex) => { deny_all_attrs_on_fields (ctx , fields) ? ; pair_regex_self (regex) } StratMode :: Arbitrary => { derive_product_has_params (ctx , ut , error :: ENUM_VARIANT_FIELD , map_closure (v_path , & fields) , fields ,) ? } } ; Ok (pair) }
};
}
