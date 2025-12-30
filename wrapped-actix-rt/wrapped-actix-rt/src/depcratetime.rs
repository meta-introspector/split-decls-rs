// Generated macro for time (module)
macro_rules! Depcratetime {
() => {
// Module: crate
// Provides: {"time"}
// Dependencies: {}
pub mod time { # ! [doc = " Utilities for tracking time (Tokio re-exports)."] pub use tokio :: time :: { interval , interval_at , sleep , sleep_until , timeout , Instant , Interval , Sleep , Timeout , } ; }
};
}
