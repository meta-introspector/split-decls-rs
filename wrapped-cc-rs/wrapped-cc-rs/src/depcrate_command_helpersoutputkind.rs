// Generated macro for OutputKind (enum)
macro_rules! Depcrate_command_helpersOutputKind {
() => {
// Module: crate::command_helpers
// Provides: {"OutputKind"}
// Dependencies: {}
# [doc = " Different strategies for handling compiler output (to stdout)"] # [derive (Clone , Debug)] pub (crate) enum OutputKind { # [doc = " Forward the output to this process' stdout ([`Stdio::inherit()`])"] Forward , # [doc = " Discard the output ([`Stdio::null()`])"] Discard , # [doc = " Capture the result (`[Stdio::piped()`])"] Capture , }
};
}
