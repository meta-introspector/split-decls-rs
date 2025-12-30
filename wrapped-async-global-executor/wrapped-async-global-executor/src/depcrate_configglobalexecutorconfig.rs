// Generated macro for GlobalExecutorConfig (struct)
macro_rules! Depcrate_configGlobalExecutorConfig {
() => {
// Module: crate::config
// Provides: {"GlobalExecutorConfig"}
// Dependencies: {}
# [doc = " Configuration to init the thread pool for the multi-threaded global executor."] # [derive (Default)] pub struct GlobalExecutorConfig { # [doc = " The environment variable from which we'll try to parse the number of threads to spawn."] env_var : Option < & 'static str > , # [doc = " The minimum number of threads to spawn."] min_threads : Option < usize > , # [doc = " The maximum number of threads to spawn."] max_threads : Option < usize > , # [doc = " The closure function used to get the name of the thread. The name can be used for identification in panic messages."] thread_name_fn : Option < Box < dyn Fn () -> String + Send + Sync > > , }
};
}
