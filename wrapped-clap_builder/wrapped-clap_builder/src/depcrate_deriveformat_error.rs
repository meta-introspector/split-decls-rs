// Generated macro for format_error (function)
macro_rules! Depcrate_deriveformat_error {
() => {
// Module: crate::derive
// Provides: {"format_error"}
// Dependencies: {}
fn format_error < I : CommandFactory > (err : Error) -> Error { let mut cmd = I :: command () ; err . format (& mut cmd) }
};
}
