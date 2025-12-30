// Generated macro for parse_skip (function)
macro_rules! Depcrate_attrparse_skip {
() => {
// Module: crate::attr
// Provides: {"parse_skip"}
// Dependencies: {}
# [doc = " Parse a skip attribute."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(skip)]`"] fn parse_skip (ctx : Ctx , acc : & mut ParseAcc , meta : Meta) { parse_bare_modifier (ctx , & mut acc . skip , meta , error :: skip_malformed) }
};
}
