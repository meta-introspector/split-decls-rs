// Generated macro for BetaAlgorithm (enum)
macro_rules! Depcrate_betaBetaAlgorithm {
() => {
// Module: crate::beta
// Provides: {"BetaAlgorithm"}
// Dependencies: {}
# [doc = " The algorithm used for sampling the Beta distribution."] # [doc = ""] # [doc = " Reference:"] # [doc = ""] # [doc = " R. C. H. Cheng (1978)."] # [doc = " Generating beta variates with nonintegral shape parameters."] # [doc = " Communications of the ACM 21, 317-322."] # [doc = " https://doi.org/10.1145/359460.359482"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] enum BetaAlgorithm < N > { BB (BB < N >) , BC (BC < N >) , }
};
}
