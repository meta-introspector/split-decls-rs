// Generated macro for PlottingBackend (enum)
macro_rules! DepcratePlottingBackend {
() => {
// Module: crate
// Provides: {"PlottingBackend"}
// Dependencies: {}
# [doc = " Enum used to select the plotting backend."] # [doc = ""] # [doc = " See [`Criterion::plotting_backend`]."] # [derive (Debug , Clone , Copy)] pub enum PlottingBackend { # [doc = " Plotting backend which uses the external `gnuplot` command to render plots. This is the"] # [doc = " default if the `gnuplot` command is installed."] Gnuplot , # [doc = " Plotting backend which uses the Rust 'Plotters' library. This is the default if `gnuplot`"] # [doc = " is not installed."] Plotters , # [doc = " Null plotting backend which outputs nothing,"] None , }
};
}
