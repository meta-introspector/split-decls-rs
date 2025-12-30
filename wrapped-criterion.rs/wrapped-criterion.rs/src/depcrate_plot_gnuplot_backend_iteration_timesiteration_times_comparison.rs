// Generated macro for iteration_times_comparison (function)
macro_rules! Depcrate_plot_gnuplot_backend_iteration_timesiteration_times_comparison {
() => {
// Module: crate::plot::gnuplot_backend::iteration_times
// Provides: {"iteration_times_comparison"}
// Dependencies: {}
pub (crate) fn iteration_times_comparison (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = iteration_times_comparison_figure (formatter , measurements , comparison , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; let path = context . report_path (id , "both/iteration_times.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
