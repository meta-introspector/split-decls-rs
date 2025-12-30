// Generated macro for clicolor_force (function)
macro_rules! Depcrateclicolor_force {
() => {
// Module: crate
// Provides: {"clicolor_force"}
// Dependencies: {}
# [doc = " Check [CLICOLOR_FORCE] status"] # [doc = ""] # [doc = " ANSI colors should be enabled no matter what."] # [doc = ""] # [doc = " [CLICOLOR_FORCE]: https://bixense.com/clicolors/"] # [inline] pub fn clicolor_force () -> bool { non_empty (std :: env :: var_os ("CLICOLOR_FORCE") . as_deref ()) }
};
}
