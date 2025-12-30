// Generated macro for SamplingMethod (enum)
macro_rules! Depcrate_hypergeometricSamplingMethod {
() => {
// Module: crate::hypergeometric
// Provides: {"SamplingMethod"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] enum SamplingMethod { InverseTransform { initial_p : f64 , initial_x : i64 , } , RejectionAcceptance { m : f64 , a : f64 , lambda_l : f64 , lambda_r : f64 , x_l : f64 , x_r : f64 , p1 : f64 , p2 : f64 , p3 : f64 , } , }
};
}
