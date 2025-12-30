// Generated macro for signal (module)
macro_rules! Depcratesignal {
() => {
// Module: crate
// Provides: {"signal"}
// Dependencies: {}
pub mod signal { # ! [doc = " Asynchronous signal handling (Tokio re-exports)."] # [cfg (unix)] pub mod unix { # ! [doc = " Unix specific signals (Tokio re-exports)."] pub use tokio :: signal :: unix :: * ; } pub use tokio :: signal :: ctrl_c ; }
};
}
