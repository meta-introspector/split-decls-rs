// Generated macro for BbrBwLoReductionStrategy (enum)
macro_rules! Depcrate_recovery_gcongestionBbrBwLoReductionStrategy {
() => {
// Module: crate::recovery::gcongestion
// Provides: {"BbrBwLoReductionStrategy"}
// Dependencies: {}
# [doc = " Controls BBR's bandwidth reduction strategy on congestion event."] # [doc = ""] # [doc = " This functionality is experimental and will be removed in the future."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (C)] # [doc (hidden)] pub enum BbrBwLoReductionStrategy { # [doc = " Uses the default strategy based on `BBRBeta`."] Default = 0 , # [doc = " Considers min-rtt to estimate bandwidth reduction."] MinRttReduction = 1 , # [doc = " Considers inflight data to estimate bandwidth reduction."] InflightReduction = 2 , # [doc = " Considers cwnd to estimate bandwidth reduction."] CwndReduction = 3 , }
};
}
