// Generated macro for FilterFn (trait)
macro_rules! Depcrate_strategy_staticsFilterFn {
() => {
// Module: crate::strategy::statics
// Provides: {"FilterFn"}
// Dependencies: {}
# [doc = " Essentially `Fn (&T) -> bool`."] pub trait FilterFn < T > { # [doc = " Test whether `t` passes the filter."] fn apply (& self , t : & T) -> bool ; }
};
}
