// Generated macro for init_from_env (function)
macro_rules! Depcrate_loggerinit_from_env {
() => {
// Module: crate::logger
// Provides: {"init_from_env"}
// Dependencies: {}
# [doc = " Initializes the global logger with an env logger from the given environment"] # [doc = " variables."] # [doc = ""] # [doc = " This should be called early in the execution of a Rust program. Any log"] # [doc = " events that occur before initialization will be ignored."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Initialise a logger using the `MY_LOG` environment variable for filters"] # [doc = " and `MY_LOG_STYLE` for writing colors:"] # [doc = ""] # [doc = " ```"] # [doc = " use env_logger::{Builder, Env};"] # [doc = ""] # [doc = " let env = Env::new().filter(\"MY_LOG\").write_style(\"MY_LOG_STYLE\");"] # [doc = ""] # [doc = " env_logger::init_from_env(env);"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if it is called more than once, or if another"] # [doc = " library has already initialized a global logger."] pub fn init_from_env < 'a , E > (env : E) where E : Into < Env < 'a > > , { try_init_from_env (env) . expect ("env_logger::init_from_env should not be called after logger initialized") ; }
};
}
