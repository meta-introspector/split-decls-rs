// Generated macro for init (function)
macro_rules! Depcrate_loggerinit {
() => {
// Module: crate::logger
// Provides: {"init"}
// Dependencies: {}
# [doc = " Initializes the global logger with an env logger."] # [doc = ""] # [doc = " This should be called early in the execution of a Rust program. Any log"] # [doc = " events that occur before initialization will be ignored."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if it is called more than once, or if another"] # [doc = " library has already initialized a global logger."] pub fn init () { try_init () . expect ("env_logger::init should not be called after logger initialized") ; }
};
}
