// Generated macro for FilterMap (struct)
macro_rules! Depcrate_strategy_filter_mapFilterMap {
() => {
// Module: crate::strategy::filter_map
// Provides: {"FilterMap"}
// Dependencies: {}
# [doc = " `Strategy` and `ValueTree` filter_map adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_filter_map()`."] # [must_use = "strategies do nothing unless used"] pub struct FilterMap < S , F > { pub (super) source : S , pub (super) whence : Reason , pub (super) fun : Arc < F > , }
};
}
