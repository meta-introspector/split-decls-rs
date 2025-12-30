// Generated macro for Estimate (struct)
macro_rules! Depcrate_estimateEstimate {
() => {
// Module: crate::estimate
// Provides: {"Estimate"}
// Dependencies: {}
# [derive (Clone , PartialEq , Deserialize , Serialize , Debug)] pub struct Estimate { # [doc = " The confidence interval for this estimate"] pub confidence_interval : ConfidenceInterval , # [doc = " The value of this estimate"] pub point_estimate : f64 , # [doc = " The standard error of this estimate"] pub standard_error : f64 , }
};
}
