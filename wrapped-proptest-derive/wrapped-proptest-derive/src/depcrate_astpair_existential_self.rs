// Generated macro for pair_existential_self (function)
macro_rules! Depcrate_astpair_existential_self {
() => {
// Module: crate::ast
// Provides: {"pair_existential_self"}
// Dependencies: {}
# [doc = " Same as `pair_existential` for the `Self` type."] pub fn pair_existential_self (strat : syn :: Expr) -> StratPair { pair_existential (self_ty () , strat) }
};
}
