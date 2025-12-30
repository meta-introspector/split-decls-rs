// Generated macro for uses_named_pipe_jobserver (function)
macro_rules! Depcrateuses_named_pipe_jobserver {
() => {
// Module: crate
// Provides: {"uses_named_pipe_jobserver"}
// Dependencies: {}
# [doc = " Returns whether the given MAKEFLAGS indicate that there is an available"] # [doc = " jobserver that uses a named pipe (fifo)"] fn uses_named_pipe_jobserver (makeflags : & OsStr) -> bool { makeflags . to_string_lossy () . contains ("--jobserver-auth=fifo:") }
};
}
