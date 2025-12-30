// Generated macro for Method (enum)
macro_rules! Depcrate_poissonMethod {
() => {
// Module: crate::poisson
// Provides: {"Method"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] enum Method < F > { Knuth (KnuthMethod < F >) , Rejection (RejectionMethod < F >) , }
};
}
