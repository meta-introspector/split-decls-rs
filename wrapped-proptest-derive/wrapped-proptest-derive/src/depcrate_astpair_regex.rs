// Generated macro for pair_regex (function)
macro_rules! Depcrate_astpair_regex {
() => {
// Module: crate::ast
// Provides: {"pair_regex"}
// Dependencies: {}
# [doc = " The type and constructor for `#[proptest(regex(..))]`."] pub fn pair_regex (ty : syn :: Type , regex : syn :: Expr) -> StratPair { (Strategy :: Regex (ty . clone ()) , Ctor :: Regex (ty , regex)) }
};
}
