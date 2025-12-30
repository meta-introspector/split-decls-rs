// Generated macro for sleep_until (function)
macro_rules! Depcrate_testsleep_until {
() => {
// Module: crate::test
// Provides: {"sleep_until"}
// Dependencies: {}
# [doc = " Calls the provided closure"] # [doc = " * If it returned true, returns true"] # [doc = " * If it returned false, waits for a short period and tries again"] # [doc = " * If the long timeout was reached, returns false"] # [doc = ""] # [doc = " It's useful for the [`PollWatcher`] due to race conditions between"] # [doc = " file system and changes detector - sometimes we can encounter an error while scanning fs,"] # [doc = " and it's hard to deal with."] # [must_use] pub fn sleep_until < F : FnMut () -> bool > (mut check : F , timeout : Duration) -> bool { let start = Instant :: now () ; loop { if Instant :: now () . checked_duration_since (start) . is_some_and (| since | since > timeout) { return false ; } if check () { return true ; } thread :: sleep (Duration :: from_millis (10)) ; } }
};
}
