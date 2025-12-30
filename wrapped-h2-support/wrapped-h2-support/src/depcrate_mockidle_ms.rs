// Generated macro for idle_ms (function)
macro_rules! Depcrate_mockidle_ms {
() => {
// Module: crate::mock
// Provides: {"idle_ms"}
// Dependencies: {}
pub async fn idle_ms (ms : u64) { tokio :: time :: sleep (Duration :: from_millis (ms)) . await }
};
}
