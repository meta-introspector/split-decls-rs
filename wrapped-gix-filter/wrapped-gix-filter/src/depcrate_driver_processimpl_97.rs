// Generated macro for impl_97 (impl)
macro_rules! Depcrate_driver_processimpl_97 {
() => {
// Module: crate::driver::process
// Provides: {"impl_97"}
// Dependencies: {}
# [doc = " Initialization"] impl Status { # [doc = " Create a new instance that represents a successful operation."] pub fn success () -> Self { Status :: Named ("success" . into ()) } # [doc = " Create a new instance that represents a delayed operation."] pub fn delayed () -> Self { Status :: Named ("delayed" . into ()) } # [doc = " Create a status that indicates to the client that the command that caused it will not be run anymore throughout the lifetime"] # [doc = " of the process. However, other commands may still run."] pub fn abort () -> Self { Status :: Named ("abort" . into ()) } # [doc = " Create a status that makes the client send a kill signal."] pub fn exit () -> Self { Status :: Named ("send-term-signal" . into ()) } # [doc = " Create a new instance that represents an error with the given `message`."] pub fn error (message : impl Into < String >) -> Self { Status :: Named (message . into ()) } }
};
}
