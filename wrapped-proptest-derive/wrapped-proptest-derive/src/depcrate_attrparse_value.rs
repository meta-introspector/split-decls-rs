// Generated macro for parse_value (function)
macro_rules! Depcrate_attrparse_value {
() => {
// Module: crate::attr
// Provides: {"parse_value"}
// Dependencies: {}
# [doc = " Parses an explicit value as a strategy."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(value = <literal>)]`"] # [doc = " + `#[proptest(value = \"<expr>\")]`"] # [doc = " + `#[proptest(value(\"<expr>\")]`"] # [doc = " + `#[proptest(value(<literal>)]`"] # [doc = " + `#[proptest(value(<ident>)]`"] fn parse_value (ctx : Ctx , acc : & mut ParseAcc , meta : & Meta) { parse_strategy_base (ctx , & mut acc . value , meta) }
};
}
