// Generated macro for pair_value_self (function)
macro_rules! Depcrate_astpair_value_self {
() => {
// Module: crate::ast
// Provides: {"pair_value_self"}
// Dependencies: {}
# [doc = " Same as `pair_value` for the `Self` type."] pub fn pair_value_self (val : syn :: Expr) -> StratPair { pair_value (self_ty () , val) }
};
}
