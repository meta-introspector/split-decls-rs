// Generated macro for impl_796 (impl)
macro_rules! Depcrate_strategy_filterimpl_796 {
() => {
// Module: crate::strategy::filter
// Provides: {"impl_796"}
// Dependencies: {}
impl < S , F > Filter < S , F > { pub (super) fn new (source : S , whence : Reason , fun : F) -> Self { Self { source , whence , fun : Arc :: new (fun) , } } }
};
}
