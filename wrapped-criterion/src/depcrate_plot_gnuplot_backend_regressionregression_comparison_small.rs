// Generated macro for regression_comparison_small (function)
macro_rules! Depcrate_plot_gnuplot_backend_regressionregression_comparison_small {
() => {
// Module: crate::plot::gnuplot_backend::regression
// Provides: {"regression_comparison_small"}
// Dependencies: {}
pub (crate) fn regression_comparison_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , base_data : & Data < '_ , f64 , f64 > , size : Option < Size > ,) -> Child { let mut figure = regression_comparison_figure (formatter , measurements , comparison , base_data , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "relative_regression_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
