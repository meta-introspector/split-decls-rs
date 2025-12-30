// Generated macro for now (function)
macro_rules! Depcrate_executor_networknow {
() => {
// Module: crate::executor::network
// Provides: {"now"}
// Dependencies: {}
# [inline] pub (crate) fn now () -> Instant { Instant :: from_micros_const (arch :: kernel :: systemtime :: now_micros () . try_into () . unwrap ()) }
};
}
