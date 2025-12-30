// Generated macro for set_max_level (function)
macro_rules! Depcrateset_max_level {
() => {
// Module: crate
// Provides: {"set_max_level"}
// Dependencies: {}
# [doc = " Sets the global maximum log level."] # [doc = ""] # [doc = " Generally, this should only be called by the active logging implementation."] # [doc = ""] # [doc = " Note that `Trace` is the maximum level, because it provides the maximum amount of detail in the emitted logs."] # [inline] # [cfg (target_has_atomic = "ptr")] pub fn set_max_level (level : LevelFilter) { MAX_LOG_LEVEL_FILTER . store (level as usize , Ordering :: Relaxed) ; }
};
}
