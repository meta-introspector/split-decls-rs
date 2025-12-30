// Generated macro for init (function)
macro_rules! Depcrate_initinit {
() => {
// Module: crate::init
// Provides: {"init"}
// Dependencies: {}
# [doc = " Init the global executor, spawning as many threads as the number or cpus or"] # [doc = " the value specified by the `ASYNC_GLOBAL_EXECUTOR_THREADS` environment variable"] # [doc = " if specified."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " async_global_executor::init();"] # [doc = " ```"] pub fn init () { static INIT_DONE : AtomicBool = AtomicBool :: new (false) ; if ! INIT_DONE . swap (true , Ordering :: SeqCst) { let config = crate :: config :: GLOBAL_EXECUTOR_CONFIG . get_or_init (crate :: config :: Config :: default) ; crate :: reactor :: block_on (async { crate :: threading :: spawn_more_threads (config . min_threads) . await . expect ("cannot spawn executor threads") ; }) ; } }
};
}
