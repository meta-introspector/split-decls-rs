// Generated macro for CongestionControlAlgorithm (enum)
macro_rules! Depcrate_recoveryCongestionControlAlgorithm {
() => {
// Module: crate::recovery
// Provides: {"CongestionControlAlgorithm"}
// Dependencies: {}
# [doc = " Available congestion control algorithms."] # [doc = ""] # [doc = " This enum provides currently available list of congestion control"] # [doc = " algorithms."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (C)] pub enum CongestionControlAlgorithm { # [doc = " Reno congestion control algorithm. `reno` in a string form."] Reno = 0 , # [doc = " CUBIC congestion control algorithm (default). `cubic` in a string form."] CUBIC = 1 , # [doc = " BBR congestion control algorithm. `bbr` in a string form."] BBR = 2 , # [doc = " BBRv2 congestion control algorithm. `bbr2` in a string form."] BBR2 = 3 , # [doc = " BBRv2 congestion control algorithm implementation from gcongestion"] # [doc = " branch. `bbr2_gcongestion` in a string form."] Bbr2Gcongestion = 4 , }
};
}
