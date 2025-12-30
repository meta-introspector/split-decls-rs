// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error is a crate's erorr type."] # [derive (Debug)] pub enum Error { # [doc = " Internal windows error."] Win (win :: Error) , # [doc = " A error which is returned in case timeout was reached."] Timeout (Duration) , # [doc = " wait for process end failed due to misc. reasons."] WaitFailed (WAIT_EVENT) , # [doc = " Input already closed"] InputClosed , }
};
}
