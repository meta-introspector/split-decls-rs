// Generated macro for cargo_cmd (function)
macro_rules! Depcrate_utilscargo_cmd {
() => {
// Module: crate::utils
// Provides: {"cargo_cmd"}
// Dependencies: {}
# [doc = " Creates a `Command` for running cargo."] # [must_use] pub fn cargo_cmd () -> Command { if let Some (path) = env :: var_os ("CARGO") { Command :: new (path) } else { Command :: new ("cargo") } }
};
}
