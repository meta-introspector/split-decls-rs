// Generated macro for rel_distributions (function)
macro_rules! Depcrate_plot_gnuplot_backend_distributionsrel_distributions {
() => {
// Module: crate::plot::gnuplot_backend::distributions
// Provides: {"rel_distributions"}
// Dependencies: {}
pub (crate) fn rel_distributions (id : & BenchmarkId , context : & ReportContext , _measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Vec < Child > { crate :: plot :: CHANGE_STATS . iter () . map (| & statistic | { rel_distribution (id , context , statistic , comparison . relative_distributions . get (statistic) , comparison . relative_estimates . get (statistic) , comparison . noise_threshold , size ,) }) . collect :: < Vec < _ > > () }
};
}
