// Generated macro for impl_808 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_808 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_808"}
// Dependencies: {}
impl < S , F > FilterMap < S , F > { pub (super) fn new (source : S , whence : Reason , fun : F) -> Self { Self { source , whence , fun : Arc :: new (fun) , } } }
};
}
