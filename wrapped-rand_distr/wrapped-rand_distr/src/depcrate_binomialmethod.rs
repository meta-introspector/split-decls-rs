// Generated macro for Method (enum)
macro_rules! Depcrate_binomialMethod {
() => {
// Module: crate::binomial
// Provides: {"Method"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] enum Method { Binv (Binv , bool) , Btpe (Btpe , bool) , Poisson (crate :: poisson :: KnuthMethod < f64 >) , Constant (u64) , }
};
}
