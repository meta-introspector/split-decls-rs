// Generated macro for t_test (function)
macro_rules! Depcrate_analysis_comparet_test {
() => {
// Module: crate::analysis::compare
// Provides: {"t_test"}
// Dependencies: {}
fn t_test (avg_times : & Sample < f64 > , base_avg_times : & Sample < f64 > , config : & BenchmarkConfig ,) -> (f64 , Distribution < f64 >) { let nresamples = config . nresamples ; let t_statistic = avg_times . t (base_avg_times) ; let t_distribution = elapsed ! ("Bootstrapping the T distribution" , mixed :: bootstrap (avg_times , base_avg_times , nresamples , | a , b | (a . t (b) ,))) . 0 ; let t_distribution = Distribution :: from (t_distribution . iter () . filter (| a | a . is_finite ()) . cloned () . collect :: < Vec < _ > > () . into_boxed_slice () ,) ; (t_statistic , t_distribution) }
};
}
