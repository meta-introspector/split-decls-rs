// Generated macro for FormatSetting (enum)
macro_rules! Depcrate_argsFormatSetting {
() => {
// Module: crate::args
// Provides: {"FormatSetting"}
// Dependencies: {}
# [doc = " Possible values for the `--format` option."] # [derive (Debug , Clone , Copy , PartialEq , Eq , ValueEnum)] pub enum FormatSetting { # [doc = " One line per test. Output for humans. (default)"] Pretty , # [doc = " One character per test. Usefull for test suites with many tests."] Terse , # [doc = " Json output"] Json , }
};
}
