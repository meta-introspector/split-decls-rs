// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_strategy_unionsimpl_1026 {
() => {
// Module: crate::strategy::unions
// Provides: {"impl_1026"}
// Dependencies: {}
impl < T > TupleUnion < T > { # [doc = " Wrap `tuple` in a `TupleUnion`."] # [doc = ""] # [doc = " The struct definition allows any `T` for `tuple`, but to be useful, it"] # [doc = " must be a 2- to 10-tuple of `(u32, Arc<impl Strategy>)` pairs where all"] # [doc = " strategies ultimately produce the same value. Each `u32` indicates the"] # [doc = " relative weight of its corresponding strategy."] # [doc = " You may use `WA<S>` as an alias for `(u32, Arc<S>)`."] # [doc = ""] # [doc = " Using this constructor directly is discouraged; prefer to use"] # [doc = " `prop_oneof!` since it is generally clearer."] pub fn new (tuple : T) -> Self { TupleUnion (tuple) } }
};
}
