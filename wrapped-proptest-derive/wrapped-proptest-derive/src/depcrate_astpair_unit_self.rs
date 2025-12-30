// Generated macro for pair_unit_self (function)
macro_rules! Depcrate_astpair_unit_self {
() => {
// Module: crate::ast
// Provides: {"pair_unit_self"}
// Dependencies: {}
# [doc = " Same as `pair_value` but for a unit variant or unit struct."] pub fn pair_unit_self (path : & syn :: Path) -> StratPair { pair_value_self (parse_quote ! (# path { })) }
};
}
