// Generated macro for pair_regex_self (function)
macro_rules! Depcrate_astpair_regex_self {
() => {
// Module: crate::ast
// Provides: {"pair_regex_self"}
// Dependencies: {}
# [doc = " Same as `pair_regex` for the `Self` type."] pub fn pair_regex_self (regex : syn :: Expr) -> StratPair { pair_regex (self_ty () , regex) }
};
}
