// Generated macro for PollTimeoutTryFromError (enum)
macro_rules! Depcrate_poll_timeoutPollTimeoutTryFromError {
() => {
// Module: crate::poll_timeout
// Provides: {"PollTimeoutTryFromError"}
// Dependencies: {}
# [doc = " Error type for integer conversions into `PollTimeout`."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum PollTimeoutTryFromError { # [doc = " Passing a value less than -1 is invalid on some systems, see"] # [doc = " <https://man.freebsd.org/cgi/man.cgi?poll#end>."] TooNegative , # [doc = " Passing a value greater than `i32::MAX` is invalid."] TooPositive , }
};
}
