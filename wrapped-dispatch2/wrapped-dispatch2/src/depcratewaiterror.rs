// Generated macro for WaitError (enum)
macro_rules! DepcrateWaitError {
() => {
// Module: crate
// Provides: {"WaitError"}
// Dependencies: {}
# [doc = " Wait error."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub enum WaitError { # [doc = " The given timeout value will result in an overflow when converting to dispatch time."] TimeOverflow , # [doc = " The operation timed out."] Timeout , }
};
}
