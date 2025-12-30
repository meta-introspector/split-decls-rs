// Generated macro for Opts (struct)
macro_rules! DepcrateOpts {
() => {
// Module: crate
// Provides: {"Opts"}
// Dependencies: {}
# [derive (clap :: Parser)] struct Opts { # [doc = " Run in retest mode"] # [arg (long)] retest : bool , # [doc = " Run regression tests"] # [arg (long)] regressions : bool , }
};
}
