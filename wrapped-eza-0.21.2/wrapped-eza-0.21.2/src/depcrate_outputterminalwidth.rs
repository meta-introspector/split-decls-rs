// Generated macro for TerminalWidth (enum)
macro_rules! Depcrate_outputTerminalWidth {
() => {
// Module: crate::output
// Provides: {"TerminalWidth"}
// Dependencies: {}
# [doc = " The width of the terminal requested by the user."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum TerminalWidth { # [doc = " The user requested this specific number of columns."] Set (usize) , # [doc = " Look up the terminal size at runtime."] Automatic , }
};
}
