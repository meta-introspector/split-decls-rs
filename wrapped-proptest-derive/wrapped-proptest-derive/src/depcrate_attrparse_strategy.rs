// Generated macro for parse_strategy (function)
macro_rules! Depcrate_attrparse_strategy {
() => {
// Module: crate::attr
// Provides: {"parse_strategy"}
// Dependencies: {}
# [doc = " Parses an explicit strategy."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(strategy = <literal>)]`"] # [doc = " + `#[proptest(strategy = \"<expr>\")]`"] # [doc = " + `#[proptest(strategy(\"<expr>\")]`"] # [doc = " + `#[proptest(strategy(<literal>)]`"] # [doc = " + `#[proptest(strategy(<ident>)]`"] fn parse_strategy (ctx : Ctx , acc : & mut ParseAcc , meta : & Meta) { parse_strategy_base (ctx , & mut acc . strategy , meta) }
};
}
