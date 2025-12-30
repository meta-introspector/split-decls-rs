// Generated macro for rel_distributions (function)
macro_rules! Depcrate_plot_plotters_backend_distributionsrel_distributions {
() => {
// Module: crate::plot::plotters_backend::distributions
// Provides: {"rel_distributions"}
// Dependencies: {}
pub (crate) fn rel_distributions (id : & BenchmarkId , context : & ReportContext , _measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < (u32 , u32) > ,) { crate :: plot :: CHANGE_STATS . iter () . for_each (| & statistic | { rel_distribution (id , context , statistic , comparison . relative_distributions . get (statistic) , comparison . relative_estimates . get (statistic) , comparison . noise_threshold , size ,) ; }) ; }
};
}
