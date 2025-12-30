// Generated macro for init_with_config (function)
macro_rules! Depcrate_initinit_with_config {
() => {
// Module: crate::init
// Provides: {"init_with_config"}
// Dependencies: {}
# [doc = " Init the global executor, spawning as many threads as specified or"] # [doc = " the value specified by the specified environment variable."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " async_global_executor::init_with_config("] # [doc = "     async_global_executor::GlobalExecutorConfig::default()"] # [doc = "         .with_env_var(\"NUMBER_OF_THREADS\")"] # [doc = "         .with_min_threads(4)"] # [doc = "         .with_max_threads(6)"] # [doc = "         .with_thread_name_fn(Box::new(|| \"worker\".to_string()))"] # [doc = " );"] # [doc = " ```"] pub fn init_with_config (config : crate :: config :: GlobalExecutorConfig) { let _ = crate :: config :: GLOBAL_EXECUTOR_CONFIG . set (config . seal ()) ; init () ; }
};
}
