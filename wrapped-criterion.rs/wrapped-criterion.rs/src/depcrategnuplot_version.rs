// Generated macro for gnuplot_version (function)
macro_rules! Depcrategnuplot_version {
() => {
// Module: crate
// Provides: {"gnuplot_version"}
// Dependencies: {}
fn gnuplot_version () -> & 'static Result < Version , VersionError > { static GNUPLOT_VERSION : OnceLock < Result < Version , VersionError > > = OnceLock :: new () ; GNUPLOT_VERSION . get_or_init (criterion_plot :: version) }
};
}
