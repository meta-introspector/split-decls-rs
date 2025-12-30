// Generated macro for gnuplot_escape (function)
macro_rules! Depcrate_plot_gnuplot_backendgnuplot_escape {
() => {
// Module: crate::plot::gnuplot_backend
// Provides: {"gnuplot_escape"}
// Dependencies: {}
fn gnuplot_escape (string : & str) -> String { string . replace ('_' , "\\_") . replace ('\'' , "''") }
};
}
