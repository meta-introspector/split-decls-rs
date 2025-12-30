// Generated macro for parse_no_bound (function)
macro_rules! Depcrate_attrparse_no_bound {
() => {
// Module: crate::attr
// Provides: {"parse_no_bound"}
// Dependencies: {}
# [doc = " Parse a no_bound attribute."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(no_bound)]`"] fn parse_no_bound (ctx : Ctx , acc : & mut ParseAcc , meta : Meta) { parse_bare_modifier (ctx , & mut acc . no_bound , meta , error :: no_bound_malformed) }
};
}
