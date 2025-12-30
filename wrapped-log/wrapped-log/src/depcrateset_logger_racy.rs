// Generated macro for set_logger_racy (function)
macro_rules! Depcrateset_logger_racy {
() => {
// Module: crate
// Provides: {"set_logger_racy"}
// Dependencies: {}
# [doc = " A thread-unsafe version of [`set_logger`]."] # [doc = ""] # [doc = " This function is available on all platforms, even those that do not have"] # [doc = " support for atomics that is needed by [`set_logger`]."] # [doc = ""] # [doc = " In almost all cases, [`set_logger`] should be preferred."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is only safe to call when it cannot race with any other"] # [doc = " calls to `set_logger` or `set_logger_racy`."] # [doc = ""] # [doc = " This can be upheld by (for example) making sure that **there are no other"] # [doc = " threads**, and (on embedded) that **interrupts are disabled**."] # [doc = ""] # [doc = " It is safe to use other logging functions while this function runs"] # [doc = " (including all logging macros)."] # [doc = ""] # [doc = " [`set_logger`]: fn.set_logger.html"] pub unsafe fn set_logger_racy (logger : & 'static dyn Log) -> Result < () , SetLoggerError > { match STATE . load (Ordering :: Acquire) { UNINITIALIZED => { LOGGER = logger ; STATE . store (INITIALIZED , Ordering :: Release) ; Ok (()) } INITIALIZING => { unreachable ! ("set_logger_racy must not be used with other initialization functions") } _ => Err (SetLoggerError (())) , } }
};
}
