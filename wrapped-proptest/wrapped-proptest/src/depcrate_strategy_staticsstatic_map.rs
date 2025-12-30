// Generated macro for static_map (function)
macro_rules! Depcrate_strategy_staticsstatic_map {
() => {
// Module: crate::strategy::statics
// Provides: {"static_map"}
// Dependencies: {}
pub (crate) fn static_map < S : Strategy , O : fmt :: Debug > (strat : S , fun : fn (S :: Value) -> O ,) -> Map < S , fn (S :: Value) -> O > { Map :: new (strat , fun) }
};
}
