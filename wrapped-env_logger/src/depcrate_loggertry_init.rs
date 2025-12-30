// Generated macro for try_init (function)
macro_rules! Depcrate_loggertry_init {
() => {
// Module: crate::logger
// Provides: {"try_init"}
// Dependencies: {}
# [doc = " Attempts to initialize the global logger with an env logger."] # [doc = ""] # [doc = " This should be called early in the execution of a Rust program. Any log"] # [doc = " events that occur before initialization will be ignored."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will fail if it is called more than once, or if another"] # [doc = " library has already initialized a global logger."] pub fn try_init () -> Result < () , SetLoggerError > { try_init_from_env (Env :: default ()) }
};
}
