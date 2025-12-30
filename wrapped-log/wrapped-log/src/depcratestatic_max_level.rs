// Generated macro for STATIC_MAX_LEVEL (const)
macro_rules! DepcrateSTATIC_MAX_LEVEL {
() => {
// Module: crate
// Provides: {"STATIC_MAX_LEVEL"}
// Dependencies: {}
# [doc = " The statically resolved maximum log level."] # [doc = ""] # [doc = " See the crate level documentation for information on how to configure this."] # [doc = ""] # [doc = " This value is checked by the log macros, but not by the `Log`ger returned by"] # [doc = " the [`logger`] function. Code that manually calls functions on that value"] # [doc = " should compare the level against this value."] # [doc = ""] # [doc = " [`logger`]: fn.logger.html"] pub const STATIC_MAX_LEVEL : LevelFilter = match cfg ! (debug_assertions) { false if cfg ! (feature = "release_max_level_off") => LevelFilter :: Off , false if cfg ! (feature = "release_max_level_error") => LevelFilter :: Error , false if cfg ! (feature = "release_max_level_warn") => LevelFilter :: Warn , false if cfg ! (feature = "release_max_level_info") => LevelFilter :: Info , false if cfg ! (feature = "release_max_level_debug") => LevelFilter :: Debug , false if cfg ! (feature = "release_max_level_trace") => LevelFilter :: Trace , _ if cfg ! (feature = "max_level_off") => LevelFilter :: Off , _ if cfg ! (feature = "max_level_error") => LevelFilter :: Error , _ if cfg ! (feature = "max_level_warn") => LevelFilter :: Warn , _ if cfg ! (feature = "max_level_info") => LevelFilter :: Info , _ if cfg ! (feature = "max_level_debug") => LevelFilter :: Debug , _ => LevelFilter :: Trace , } ;
};
}
