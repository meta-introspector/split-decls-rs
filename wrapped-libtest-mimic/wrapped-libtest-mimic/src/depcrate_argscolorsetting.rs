// Generated macro for ColorSetting (enum)
macro_rules! Depcrate_argsColorSetting {
() => {
// Module: crate::args
// Provides: {"ColorSetting"}
// Dependencies: {}
# [doc = " Possible values for the `--color` option."] # [derive (Debug , Clone , Copy , PartialEq , Eq , ValueEnum)] pub enum ColorSetting { # [doc = " Colorize output if stdout is a tty and tests are run on serially"] # [doc = " (default)."] Auto , # [doc = " Always colorize output."] Always , # [doc = " Never colorize output."] Never , }
};
}
