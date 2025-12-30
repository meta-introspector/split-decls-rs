// Generated macro for TimerError (enum)
macro_rules! Depcrate_errorTimerError {
() => {
// Module: crate::error
// Provides: {"TimerError"}
// Dependencies: {}
# [doc = " An error that can occur when [`JitterRng::test_timer`] fails."] # [doc = ""] # [doc = " All variants have a value of 0xAE530400 = 2924676096 plus a small"] # [doc = " increment (1 through 5)."] # [doc = ""] # [doc = " [`JitterRng::test_timer`]: crate::JitterRng::test_timer"] # [derive (Debug , Clone , PartialEq , Eq)] # [repr (u32)] # [allow (clippy :: manual_non_exhaustive)] pub enum TimerError { # [doc = " No timer available."] NoTimer = ERROR_BASE + 1 , # [doc = " Timer too coarse to use as an entropy source."] CoarseTimer = ERROR_BASE + 2 , # [doc = " Timer is not monotonically increasing."] NotMonotonic = ERROR_BASE + 3 , # [doc = " Variations of deltas of time too small."] TinyVariations = ERROR_BASE + 4 , # [doc = " Too many stuck results (indicating no added entropy)."] TooManyStuck = ERROR_BASE + 5 , # [doc (hidden)] __Nonexhaustive , }
};
}
