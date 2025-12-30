// Generated macro for connect (function)
macro_rules! Depcrate_pipeconnect {
() => {
// Module: crate::pipe
// Provides: {"connect"}
// Dependencies: {}
# [doc = " A convenience function to connect to a named pipe."] # [doc = ""] # [doc = " This function will block the calling process until it can connect to the"] # [doc = " pipe server specified by `addr`. This will use `NamedPipe::wait` internally"] # [doc = " to block until it can connect."] pub fn connect < A : AsRef < OsStr > > (addr : A) -> io :: Result < File > { _connect (addr . as_ref ()) }
};
}
