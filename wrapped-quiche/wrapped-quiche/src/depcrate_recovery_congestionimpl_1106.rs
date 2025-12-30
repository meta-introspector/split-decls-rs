// Generated macro for impl_1106 (impl)
macro_rules! Depcrate_recovery_congestionimpl_1106 {
() => {
// Module: crate::recovery::congestion
// Provides: {"impl_1106"}
// Dependencies: {}
impl From < CongestionControlAlgorithm > for & 'static CongestionControlOps { fn from (algo : CongestionControlAlgorithm) -> Self { match algo { CongestionControlAlgorithm :: Reno => & reno :: RENO , CongestionControlAlgorithm :: CUBIC => & cubic :: CUBIC , CongestionControlAlgorithm :: BBR => & bbr :: BBR , CongestionControlAlgorithm :: BBR2 => & bbr2 :: BBR2 , CongestionControlAlgorithm :: Bbr2Gcongestion => { debug_panic ! ("legacy implementation, not gcongestion") ; & bbr2 :: BBR2 } , } } }
};
}
