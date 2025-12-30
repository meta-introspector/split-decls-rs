// Generated macro for Filter (struct)
macro_rules! Depcrate_strategy_filterFilter {
() => {
// Module: crate::strategy::filter
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " `Strategy` and `ValueTree` filter adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_filter()`."] # [must_use = "strategies do nothing unless used"] pub struct Filter < S , F > { pub (super) source : S , pub (super) whence : Reason , pub (super) fun : Arc < F > , }
};
}
