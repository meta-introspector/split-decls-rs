// Generated macro for set_logger_inner (function)
macro_rules! Depcrateset_logger_inner {
() => {
// Module: crate
// Provides: {"set_logger_inner"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] fn set_logger_inner < F > (make_logger : F) -> Result < () , SetLoggerError > where F : FnOnce () -> & 'static dyn Log , { match STATE . compare_exchange (UNINITIALIZED , INITIALIZING , Ordering :: Acquire , Ordering :: Relaxed ,) { Ok (UNINITIALIZED) => { unsafe { LOGGER = make_logger () ; } STATE . store (INITIALIZED , Ordering :: Release) ; Ok (()) } Err (INITIALIZING) => { while STATE . load (Ordering :: Relaxed) == INITIALIZING { std :: hint :: spin_loop () ; } Err (SetLoggerError (())) } _ => Err (SetLoggerError (())) , } }
};
}
