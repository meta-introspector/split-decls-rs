// Generated macro for colors_enabled (function)
macro_rules! Depcrate_utilscolors_enabled {
() => {
// Module: crate::utils
// Provides: {"colors_enabled"}
// Dependencies: {}
# [doc = " Returns `true` if colors should be enabled for stdout."] # [doc = ""] # [doc = " This honors the [clicolors spec](http://bixense.com/clicolors/)."] # [doc = ""] # [doc = " * `CLICOLOR != 0`: ANSI colors are supported and should be used when the program isn't piped."] # [doc = " * `CLICOLOR == 0`: Don't output ANSI color escape codes."] # [doc = " * `CLICOLOR_FORCE != 0`: ANSI colors should be enabled no matter what."] # [inline] pub fn colors_enabled () -> bool { STDOUT_COLORS . load (Ordering :: Relaxed) }
};
}
