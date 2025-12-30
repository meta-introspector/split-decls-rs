// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Represents HTTP/2 operation errors."] # [doc = ""] # [doc = " `Error` covers error cases raised by protocol errors caused by the"] # [doc = " peer, I/O (transport) errors, and errors caused by the user of the library."] # [doc = ""] # [doc = " If the error was caused by the remote peer, then it will contain a"] # [doc = " [`Reason`] which can be obtained with the [`reason`] function."] # [doc = ""] # [doc = " [`Reason`]: struct.Reason.html"] # [doc = " [`reason`]: #method.reason"] # [derive (Debug)] pub struct Error { kind : Kind , }
};
}
