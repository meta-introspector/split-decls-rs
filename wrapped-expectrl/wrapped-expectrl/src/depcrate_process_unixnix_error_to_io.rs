// Generated macro for nix_error_to_io (function)
macro_rules! Depcrate_process_unixnix_error_to_io {
() => {
// Module: crate::process::unix
// Provides: {"nix_error_to_io"}
// Dependencies: {}
fn nix_error_to_io (err : nix :: Error) -> io :: Error { io :: Error :: other (err) }
};
}
