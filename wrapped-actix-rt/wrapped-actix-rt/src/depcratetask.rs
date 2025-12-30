// Generated macro for task (module)
macro_rules! Depcratetask {
() => {
// Module: crate
// Provides: {"task"}
// Dependencies: {}
pub mod task { # ! [doc = " Task management (Tokio re-exports)."] pub use tokio :: task :: { spawn_blocking , yield_now , JoinError , JoinHandle } ; }
};
}
