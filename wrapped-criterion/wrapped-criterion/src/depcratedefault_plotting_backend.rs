// Generated macro for default_plotting_backend (function)
macro_rules! Depcratedefault_plotting_backend {
() => {
// Module: crate
// Provides: {"default_plotting_backend"}
// Dependencies: {}
fn default_plotting_backend () -> & 'static PlottingBackend { static DEFAULT_PLOTTING_BACKEND : OnceLock < PlottingBackend > = OnceLock :: new () ; DEFAULT_PLOTTING_BACKEND . get_or_init (| | match gnuplot_version () { Ok (_) => PlottingBackend :: Gnuplot , # [cfg (feature = "plotters")] Err (e) => { match e { VersionError :: Exec (_) => eprintln ! ("Gnuplot not found, using plotters backend") , e => eprintln ! ("Gnuplot not found or not usable, using plotters backend\n{}" , e) , } ; PlottingBackend :: Plotters } # [cfg (not (feature = "plotters"))] Err (_) => PlottingBackend :: None , }) }
};
}
