// Generated macro for regression (function)
macro_rules! Depcrate_analysisregression {
() => {
// Module: crate::analysis
// Provides: {"regression"}
// Dependencies: {}
fn regression (data : & Data < '_ , f64 , f64 > , config : & BenchmarkConfig ,) -> (Distribution < f64 > , Estimate) { let cl = config . confidence_level ; let distribution = elapsed ! ("Bootstrapped linear regression" , data . bootstrap (config . nresamples , | d | (Slope :: fit (& d) . 0 ,))) . 0 ; let point = Slope :: fit (data) ; let (lb , ub) = distribution . confidence_interval (config . confidence_level) ; let se = distribution . std_dev (None) ; (distribution , Estimate { confidence_interval : ConfidenceInterval { confidence_level : cl , lower_bound : lb , upper_bound : ub , } , point_estimate : point . 0 , standard_error : se , } ,) }
};
}
