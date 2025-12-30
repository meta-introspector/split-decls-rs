// Generated macro for set_boxed_logger (function)
macro_rules! Depcrateset_boxed_logger {
() => {
// Module: crate
// Provides: {"set_boxed_logger"}
// Dependencies: {}
# [doc = " Sets the global logger to a `Box<Log>`."] # [doc = ""] # [doc = " This is a simple convenience wrapper over `set_logger`, which takes a"] # [doc = " `Box<Log>` rather than a `&'static Log`. See the documentation for"] # [doc = " [`set_logger`] for more details."] # [doc = ""] # [doc = " Requires the `std` feature."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error is returned if a logger has already been set."] # [doc = ""] # [doc = " [`set_logger`]: fn.set_logger.html"] # [cfg (all (feature = "std" , target_has_atomic = "ptr"))] pub fn set_boxed_logger (logger : Box < dyn Log >) -> Result < () , SetLoggerError > { set_logger_inner (| | Box :: leak (logger)) }
};
}
