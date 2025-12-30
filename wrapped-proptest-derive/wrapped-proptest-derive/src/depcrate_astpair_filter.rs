// Generated macro for pair_filter (function)
macro_rules! Depcrate_astpair_filter {
() => {
// Module: crate::ast
// Provides: {"pair_filter"}
// Dependencies: {}
# [doc = " Potentially apply a filter to a strategy type and its constructor."] pub fn pair_filter (filter : Vec < syn :: Expr > , ty : syn :: Type , pair : StratPair ,) -> StratPair { filter . into_iter () . fold (pair , | (strat , ctor) , filter | { (Strategy :: Filter (Box :: new (strat) , ty . clone ()) , Ctor :: Filter (Box :: new (ctor) , filter) ,) }) }
};
}
