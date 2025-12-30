// Generated macro for impl_921 (impl)
macro_rules! Depcrate_patimpl_921 {
() => {
// Module: crate::pat
// Provides: {"impl_921"}
// Dependencies: {}
impl AdjustMode { const fn peel_until_adt (opt_adt_def : Option < DefId >) -> AdjustMode { AdjustMode :: Peel { kind : PeelKind :: Implicit { until_adt : opt_adt_def , pat_ref_layers : 0 } } } const fn peel_all () -> AdjustMode { AdjustMode :: peel_until_adt (None) } }
};
}
