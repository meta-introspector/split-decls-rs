// Generated macro for Result (type)
macro_rules! Depcrate_errorResult {
() => {
// Module: crate::error
// Provides: {"Result"}
// Dependencies: {}
# [doc = " Alias for a [`Result`](stdResult) containing [`dbus::Error`](Error) by default."] # [doc = ""] # [doc = " Can still be used with different error types."] pub type Result < T , E = Error > = stdResult < T , E > ;
};
}
