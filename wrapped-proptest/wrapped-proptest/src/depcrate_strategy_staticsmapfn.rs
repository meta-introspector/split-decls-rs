// Generated macro for MapFn (trait)
macro_rules! Depcrate_strategy_staticsMapFn {
() => {
// Module: crate::strategy::statics
// Provides: {"MapFn"}
// Dependencies: {}
# [doc = " Essentially `Fn (T) -> Output`."] pub trait MapFn < T > { # [allow (missing_docs)] type Output : fmt :: Debug ; # [doc = " Map `T` to `Output`."] fn apply (& self , t : T) -> Self :: Output ; }
};
}
