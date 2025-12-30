// Generated macro for build_change_estimates (function)
macro_rules! Depcrate_estimatebuild_change_estimates {
() => {
// Module: crate::estimate
// Provides: {"build_change_estimates"}
// Dependencies: {}
pub fn build_change_estimates (distributions : & ChangeDistributions , points : & ChangePointEstimates , cl : f64 ,) -> ChangeEstimates { let to_estimate = | point_estimate , distribution : & Distribution < f64 > | { let (lb , ub) = distribution . confidence_interval (cl) ; Estimate { confidence_interval : ConfidenceInterval { confidence_level : cl , lower_bound : lb , upper_bound : ub , } , point_estimate , standard_error : distribution . std_dev (None) , } } ; ChangeEstimates { mean : to_estimate (points . mean , & distributions . mean) , median : to_estimate (points . median , & distributions . median) , } }
};
}
