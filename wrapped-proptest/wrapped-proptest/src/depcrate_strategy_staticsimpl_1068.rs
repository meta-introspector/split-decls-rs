// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1068 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1068"}
// Dependencies: {}
impl < S , F > Filter < S , F > { # [doc = " Adapt strategy `source` to reject values which do not pass `filter`,"] # [doc = " using `whence` as the reported reason/location."] pub fn new (source : S , whence : Reason , filter : F) -> Self { Filter { source , whence , fun : filter , } } }
};
}
