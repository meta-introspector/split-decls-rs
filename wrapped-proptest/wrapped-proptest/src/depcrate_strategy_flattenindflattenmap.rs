// Generated macro for IndFlattenMap (struct)
macro_rules! Depcrate_strategy_flattenIndFlattenMap {
() => {
// Module: crate::strategy::flatten
// Provides: {"IndFlattenMap"}
// Dependencies: {}
# [doc = " Similar to `Map` plus `Flatten`, but does not shrink the input strategy and"] # [doc = " passes the original input through."] # [doc = ""] # [doc = " See `Strategy::prop_ind_flat_map2()` for more details."] pub struct IndFlattenMap < S , F > { pub (super) source : S , pub (super) fun : Arc < F > , }
};
}
