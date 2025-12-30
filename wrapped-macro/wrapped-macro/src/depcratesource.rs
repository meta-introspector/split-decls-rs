// Generated macro for Source (enum)
macro_rules! DepcrateSource {
() => {
// Module: crate
// Provides: {"Source"}
// Dependencies: {}
# [doc = " The source of the wit package definition"] enum Source { # [doc = " A list of paths to wit directories"] Paths (Vec < PathBuf >) , # [doc = " Inline sources have an optional path to a directory of their dependencies"] Inline (String , Option < Vec < PathBuf > >) , }
};
}
