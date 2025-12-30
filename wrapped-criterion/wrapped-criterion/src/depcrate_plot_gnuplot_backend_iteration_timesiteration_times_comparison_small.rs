// Generated macro for iteration_times_comparison_small (function)
macro_rules! Depcrate_plot_gnuplot_backend_iteration_timesiteration_times_comparison_small {
() => {
// Module: crate::plot::gnuplot_backend::iteration_times
// Provides: {"iteration_times_comparison_small"}
// Dependencies: {}
pub (crate) fn iteration_times_comparison_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = iteration_times_comparison_figure (formatter , measurements , comparison , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "relative_iteration_times_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
