// Generated macro for from_exit (function)
macro_rules! Depcrate_errorfrom_exit {
() => {
// Module: crate::error
// Provides: {"from_exit"}
// Dependencies: {}
pub fn from_exit (status : process :: ExitStatus) -> Error { Error { kind : ErrorKind :: Process (status) , } }
};
}
