// Generated macro for parse_no_params (function)
macro_rules! Depcrate_attrparse_no_params {
() => {
// Module: crate::attr
// Provides: {"parse_no_params"}
// Dependencies: {}
# [doc = " Parses an order to use the default Parameters type and value."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(no_params)]`"] fn parse_no_params (ctx : Ctx , acc : & mut ParseAcc , meta : Meta) { parse_bare_modifier (ctx , & mut acc . no_params , meta , error :: no_params_malformed ,) }
};
}
