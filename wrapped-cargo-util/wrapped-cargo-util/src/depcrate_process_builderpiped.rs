// Generated macro for piped (function)
macro_rules! Depcrate_process_builderpiped {
() => {
// Module: crate::process_builder
// Provides: {"piped"}
// Dependencies: {}
# [doc = " Creates new pipes for stderr, stdout, and optionally stdin."] fn piped (cmd : & mut Command , pipe_stdin : bool) -> & mut Command { cmd . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . stdin (if pipe_stdin { Stdio :: piped () } else { Stdio :: null () }) }
};
}
