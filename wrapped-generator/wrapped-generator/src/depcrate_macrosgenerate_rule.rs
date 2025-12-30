// Generated macro for generate_rule (macro)
macro_rules! Depcrate_macrosgenerate_rule {
() => {
// Module: crate::macros
// Provides: {"generate_rule"}
// Dependencies: {}
# [cfg (not (feature = "std"))] macro_rules ! generate_rule { ($ name : ident , $ pattern : expr) => { quote ! { # [inline] # [allow (dead_code , non_snake_case , unused_variables)] pub fn $ name (state : :: alloc :: boxed :: Box <:: pest :: ParserState <'_ , Rule >>) -> :: pest :: ParseResult <:: alloc :: boxed :: Box <:: pest :: ParserState <'_ , Rule >>> { $ pattern } } } }
};
}
