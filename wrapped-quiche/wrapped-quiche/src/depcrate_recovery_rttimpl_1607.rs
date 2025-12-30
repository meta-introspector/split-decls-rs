// Generated macro for impl_1607 (impl)
macro_rules! Depcrate_recovery_rttimpl_1607 {
() => {
// Module: crate::recovery::rtt
// Provides: {"impl_1607"}
// Dependencies: {}
impl std :: fmt :: Debug for RttStats { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("RttStats") . field ("lastest_rtt" , & self . latest_rtt) . field ("srtt" , & self . smoothed_rtt) . field ("minrtt" , & * self . min_rtt) . field ("rttvar" , & self . rttvar) . finish () } }
};
}
