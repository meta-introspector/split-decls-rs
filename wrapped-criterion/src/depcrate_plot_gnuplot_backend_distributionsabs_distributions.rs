// Generated macro for abs_distributions (function)
macro_rules! Depcrate_plot_gnuplot_backend_distributionsabs_distributions {
() => {
// Module: crate::plot::gnuplot_backend::distributions
// Provides: {"abs_distributions"}
// Dependencies: {}
pub (crate) fn abs_distributions (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Vec < Child > { crate :: plot :: REPORT_STATS . iter () . filter_map (| stat | { measurements . distributions . get (* stat) . and_then (| dist | { measurements . absolute_estimates . get (* stat) . map (| est | (* stat , dist , est)) }) }) . map (| (statistic , distribution , estimate) | { abs_distribution (id , context , formatter , statistic , distribution , estimate , size ,) }) . collect :: < Vec < _ > > () }
};
}
