// Generated macro for TimeSource (trait)
macro_rules! Depcrate_configTimeSource {
() => {
// Module: crate::config
// Provides: {"TimeSource"}
// Dependencies: {}
# [doc = " Object to get current [`SystemTime`]"] # [doc = ""] # [doc = " This exists to allow system time to be mocked in tests, or wherever else desired."] pub trait TimeSource : Send + Sync { # [doc = " Get [`SystemTime::now()`](SystemTime::now) or the mocked equivalent"] fn now (& self) -> SystemTime ; }
};
}
