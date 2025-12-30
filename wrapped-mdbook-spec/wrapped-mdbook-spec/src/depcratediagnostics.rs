// Generated macro for Diagnostics (struct)
macro_rules! DepcrateDiagnostics {
() => {
// Module: crate
// Provides: {"Diagnostics"}
// Dependencies: {}
# [doc = " Handler for errors and warnings."] pub struct Diagnostics { # [doc = " Whether or not warnings should be errors (set by SPEC_DENY_WARNINGS"] # [doc = " environment variable)."] deny_warnings : bool , # [doc = " Number of messages generated."] count : u32 , }
};
}
