// Generated macro for pair_oneof (function)
macro_rules! Depcrate_astpair_oneof {
() => {
// Module: crate::ast
// Provides: {"pair_oneof"}
// Dependencies: {}
# [doc = " The type and constructor for a union of strategies which produces a new"] # [doc = " strategy that used the given strategies with probabilities based on the"] # [doc = " assigned relative weights for each strategy."] pub fn pair_oneof ((strats , ctors) : (Vec < Strategy > , Vec < (u32 , Ctor) >) ,) -> StratPair { (Strategy :: Union (strats . into ()) , Ctor :: Union (ctors . into ())) }
};
}
