// Generated macro for Map (struct)
macro_rules! Depcrate_strategy_mapMap {
() => {
// Module: crate::strategy::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " `Strategy` and `ValueTree` map adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_map()`."] # [must_use = "strategies do nothing unless used"] pub struct Map < S , F > { pub (super) source : S , pub (super) fun : Arc < F > , }
};
}
