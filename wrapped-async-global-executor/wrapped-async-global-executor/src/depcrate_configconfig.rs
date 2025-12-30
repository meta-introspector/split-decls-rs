// Generated macro for Config (struct)
macro_rules! Depcrate_configConfig {
() => {
// Module: crate::config
// Provides: {"Config"}
// Dependencies: {}
pub (crate) struct Config { pub (crate) min_threads : usize , pub (crate) max_threads : usize , pub (crate) thread_name_fn : Box < dyn Fn () -> String + Send + Sync > , }
};
}
