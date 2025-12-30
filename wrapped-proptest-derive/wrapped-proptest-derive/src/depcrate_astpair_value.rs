// Generated macro for pair_value (function)
macro_rules! Depcrate_astpair_value {
() => {
// Module: crate::ast
// Provides: {"pair_value"}
// Dependencies: {}
# [doc = " The type and constructor for a strategy that always returns the value"] # [doc = " provided in the expression `val`."] # [doc = " This is statically dispatched since no erasure is needed or used."] pub fn pair_value (ty : syn :: Type , val : syn :: Expr) -> StratPair { (Strategy :: Value (ty) , Ctor :: Value (val)) }
};
}
