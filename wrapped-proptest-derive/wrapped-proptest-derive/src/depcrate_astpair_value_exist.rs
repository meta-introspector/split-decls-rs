// Generated macro for pair_value_exist (function)
macro_rules! Depcrate_astpair_value_exist {
() => {
// Module: crate::ast
// Provides: {"pair_value_exist"}
// Dependencies: {}
# [doc = " Erased strategy for a fixed value."] pub fn pair_value_exist (ty : syn :: Type , strat : syn :: Expr) -> StratPair { (Strategy :: Existential (ty) , Ctor :: ValueExistential (strat)) }
};
}
