// Generated macro for pair_existential (function)
macro_rules! Depcrate_astpair_existential {
() => {
// Module: crate::ast
// Provides: {"pair_existential"}
// Dependencies: {}
# [doc = " The type and constructor for a specific strategy value constructed by the"] # [doc = " given expression. Currently, the type is erased and a `BoxedStrategy<Type>`"] # [doc = " is given back instead."] # [doc = ""] # [doc = " This is a temporary restriction. Once `impl Trait` is stabilized,"] # [doc = " the boxing and dynamic dispatch can be replaced with a statically"] # [doc = " dispatched anonymous type instead."] pub fn pair_existential (ty : syn :: Type , strat : syn :: Expr) -> StratPair { (Strategy :: Existential (ty) , Ctor :: Existential (strat)) }
};
}
