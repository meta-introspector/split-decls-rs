// Generated macro for regression_small (function)
macro_rules! Depcrate_plot_gnuplot_backend_regressionregression_small {
() => {
// Module: crate::plot::gnuplot_backend::regression
// Provides: {"regression_small"}
// Dependencies: {}
pub (crate) fn regression_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Child { let mut figure = regression_figure (formatter , measurements , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "regression_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
