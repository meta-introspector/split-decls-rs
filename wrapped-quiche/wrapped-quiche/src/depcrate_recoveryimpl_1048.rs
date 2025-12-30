// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_recoveryimpl_1048 {
() => {
// Module: crate::recovery
// Provides: {"impl_1048"}
// Dependencies: {}
impl FromStr for CongestionControlAlgorithm { type Err = crate :: Error ; # [doc = " Converts a string to `CongestionControlAlgorithm`."] # [doc = ""] # [doc = " If `name` is not valid, `Error::CongestionControl` is returned."] fn from_str (name : & str) -> std :: result :: Result < Self , Self :: Err > { match name { "reno" => Ok (CongestionControlAlgorithm :: Reno) , "cubic" => Ok (CongestionControlAlgorithm :: CUBIC) , "bbr" => Ok (CongestionControlAlgorithm :: BBR) , # [cfg (not (feature = "gcongestion"))] "bbr2" => Ok (CongestionControlAlgorithm :: BBR2) , # [cfg (feature = "gcongestion")] "bbr2" => Ok (CongestionControlAlgorithm :: Bbr2Gcongestion) , "bbr2_gcongestion" => Ok (CongestionControlAlgorithm :: Bbr2Gcongestion) , _ => Err (crate :: Error :: CongestionControl) , } } }
};
}
