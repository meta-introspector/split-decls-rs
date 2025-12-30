// Generated macro for impl_1599 (impl)
macro_rules! Depcrate_recovery_gcongestionimpl_1599 {
() => {
// Module: crate::recovery::gcongestion
// Provides: {"impl_1599"}
// Dependencies: {}
# [doc (hidden)] impl FromStr for BbrBwLoReductionStrategy { type Err = crate :: Error ; # [doc = " Converts a string to `BbrBwLoReductionStrategy`."] # [doc = ""] # [doc = " If `name` is not valid, `Error::CongestionControl` is returned."] fn from_str (name : & str) -> Result < Self , Self :: Err > { match name { "default" => Ok (BbrBwLoReductionStrategy :: Default) , "minrtt" => Ok (BbrBwLoReductionStrategy :: MinRttReduction) , "inflight" => Ok (BbrBwLoReductionStrategy :: InflightReduction) , "cwnd" => Ok (BbrBwLoReductionStrategy :: CwndReduction) , _ => Err (crate :: Error :: CongestionControl) , } } }
};
}
