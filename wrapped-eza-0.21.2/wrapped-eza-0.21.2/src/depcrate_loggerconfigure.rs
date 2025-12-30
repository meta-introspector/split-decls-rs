// Generated macro for configure (function)
macro_rules! Depcrate_loggerconfigure {
() => {
// Module: crate::logger
// Provides: {"configure"}
// Dependencies: {}
# [doc = " Sets the internal logger, changing the log level based on the value of an"] # [doc = " environment variable."] pub fn configure < T : AsRef < OsStr > > (ev : Option < T >) { let Some (ev) = ev else { return } ; let env_var = ev . as_ref () ; if env_var . is_empty () { return ; } if env_var == "trace" { log :: set_max_level (log :: LevelFilter :: Trace) ; } else { log :: set_max_level (log :: LevelFilter :: Debug) ; } let result = log :: set_logger (GLOBAL_LOGGER) ; if let Err (e) = result { eprintln ! ("Failed to initialize logger: {e}") ; } }
};
}
