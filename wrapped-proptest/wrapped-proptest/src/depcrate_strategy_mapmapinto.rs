// Generated macro for MapInto (struct)
macro_rules! Depcrate_strategy_mapMapInto {
() => {
// Module: crate::strategy::map
// Provides: {"MapInto"}
// Dependencies: {}
# [doc = " `Strategy` and `ValueTree` map into adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_map_into()`."] # [must_use = "strategies do nothing unless used"] pub struct MapInto < S , O > { pub (super) source : S , pub (super) output : PhantomData < O > , }
};
}
