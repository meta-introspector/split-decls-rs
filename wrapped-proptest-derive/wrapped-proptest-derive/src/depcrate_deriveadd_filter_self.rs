// Generated macro for add_filter_self (function)
macro_rules! Depcrate_deriveadd_filter_self {
() => {
// Module: crate::derive
// Provides: {"add_filter_self"}
// Dependencies: {}
# [doc = " Apply a filter with `Self` as the input type to the predicate."] fn add_filter_self (filter : Vec < Expr > , pair : StratPair) -> StratPair { pair_filter (filter , self_ty () , pair) }
};
}
