// Generated macro for impl_1505 (impl)
macro_rules! Depcrate_recovery_gcongestion_bbr2impl_1505 {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"impl_1505"}
// Dependencies: {}
impl From < BbrBwLoReductionStrategy > for BwLoMode { fn from (value : BbrBwLoReductionStrategy) -> Self { match value { BbrBwLoReductionStrategy :: Default => BwLoMode :: Default , BbrBwLoReductionStrategy :: MinRttReduction => BwLoMode :: MinRttReduction , BbrBwLoReductionStrategy :: InflightReduction => BwLoMode :: InflightReduction , BbrBwLoReductionStrategy :: CwndReduction => BwLoMode :: CwndReduction , } } }
};
}
