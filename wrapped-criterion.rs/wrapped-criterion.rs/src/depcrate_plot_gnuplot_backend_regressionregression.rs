// Generated macro for regression (function)
macro_rules! Depcrate_plot_gnuplot_backend_regressionregression {
() => {
// Module: crate::plot::gnuplot_backend::regression
// Provides: {"regression"}
// Dependencies: {}
pub (crate) fn regression (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Child { let mut figure = regression_figure (formatter , measurements , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; figure . configure (Key , | k | { k . set (Justification :: Left) . set (Order :: SampleText) . set (Position :: Inside (Vertical :: Top , Horizontal :: Left)) }) ; let path = context . report_path (id , "regression.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
