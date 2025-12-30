// Generated macro for cargo_makeflags (function)
macro_rules! Depcrate_inputcargo_makeflags {
() => {
// Module: crate::input
// Provides: {"cargo_makeflags"}
// Dependencies: {}
# [doc = " Contains parameters needed for Cargo’s [jobserver] implementation to parallelize"] # [doc = " subprocesses."] # [doc = ""] # [doc = " Rustc or cargo invocations from build.rs can already read"] # [doc = " `CARGO_MAKEFLAGS`, but GNU Make requires the flags to be specified either"] # [doc = " directly as arguments, or through the `MAKEFLAGS` environment variable."] # [doc = " Currently Cargo doesn’t set the `MAKEFLAGS` variable, but it’s free for build"] # [doc = " scripts invoking GNU Make to set it to the contents of `CARGO_MAKEFLAGS`."] # [doc = ""] # [doc = " [jobserver]: https://www.gnu.org/software/make/manual/html_node/Job-Slots.html"] # [track_caller] pub fn cargo_makeflags () -> Option < String > { ENV . get ("CARGO_MAKEFLAGS") . map (to_string) }
};
}
