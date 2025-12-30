// Generated macro for if_present_on_unit_variant (function)
macro_rules! Depcrate_errorif_present_on_unit_variant {
() => {
// Module: crate::error
// Provides: {"if_present_on_unit_variant"}
// Dependencies: {}
# [doc = " Ensures that a strategy, value, params, filter is not present on a unit variant."] pub fn if_present_on_unit_variant (ctx : Ctx , attrs : & ParsedAttributes) { # [doc = " Ensures that an explicit strategy or value is not present on a unit variant."] use crate :: attr :: StratMode :: * ; match attrs . strategy { Arbitrary => { } Strategy (_) => strategy_on_unit_variant (ctx , "strategy") , Value (_) => strategy_on_unit_variant (ctx , "value") , Regex (_) => regex_on_unit_variant (ctx) , } if attrs . params . is_set () { params_on_unit_variant (ctx) } if ! attrs . filter . is_empty () { filter_on_unit_variant (ctx) } }
};
}
