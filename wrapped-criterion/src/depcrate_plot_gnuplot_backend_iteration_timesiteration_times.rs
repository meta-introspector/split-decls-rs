// Generated macro for iteration_times (function)
macro_rules! Depcrate_plot_gnuplot_backend_iteration_timesiteration_times {
() => {
// Module: crate::plot::gnuplot_backend::iteration_times
// Provides: {"iteration_times"}
// Dependencies: {}
pub (crate) fn iteration_times (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Child { let mut figure = iteration_times_figure (formatter , measurements , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; figure . configure (Key , | k | { k . set (Justification :: Left) . set (Order :: SampleText) . set (Position :: Inside (Vertical :: Top , Horizontal :: Left)) }) ; let path = context . report_path (id , "iteration_times.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
