// Generated macro for Level (enum)
macro_rules! Depcrate_diagnosticLevel {
() => {
// Module: crate::diagnostic
// Provides: {"Level"}
// Dependencies: {}
# [doc = " An enum representing a diagnostic level."] # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] # [derive (Copy , Clone , Debug)] # [non_exhaustive] pub enum Level { # [doc = " An error."] Error , # [doc = " A warning."] Warning , # [doc = " A note."] Note , # [doc = " A help message."] Help , }
};
}
