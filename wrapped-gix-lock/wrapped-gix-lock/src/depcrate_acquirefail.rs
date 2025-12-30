// Generated macro for Fail (enum)
macro_rules! Depcrate_acquireFail {
() => {
// Module: crate::acquire
// Provides: {"Fail"}
// Dependencies: {}
# [doc = " Describe what to do if a lock cannot be obtained as it's already held elsewhere."] # [derive (Default , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub enum Fail { # [doc = " Fail after the first unsuccessful attempt of obtaining a lock."] # [default] Immediately , # [doc = " Retry after failure with quadratically longer sleep times to block the current thread."] # [doc = " Fail once the given duration is exceeded, similar to [Fail::Immediately]"] AfterDurationWithBackoff (Duration) , }
};
}
