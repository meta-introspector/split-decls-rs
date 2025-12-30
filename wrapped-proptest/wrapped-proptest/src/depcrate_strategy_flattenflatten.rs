// Generated macro for Flatten (struct)
macro_rules! Depcrate_strategy_flattenFlatten {
() => {
// Module: crate::strategy::flatten
// Provides: {"Flatten"}
// Dependencies: {}
# [doc = " Adaptor that flattens a `Strategy` which produces other `Strategy`s into a"] # [doc = " `Strategy` that picks one of those strategies and then picks values from"] # [doc = " it."] # [derive (Debug , Clone , Copy)] # [must_use = "strategies do nothing unless used"] pub struct Flatten < S > { source : S , }
};
}
