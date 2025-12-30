// Generated macro for max_level (function)
macro_rules! Depcratemax_level {
() => {
// Module: crate
// Provides: {"max_level"}
// Dependencies: {}
# [doc = " Returns the current maximum log level."] # [doc = ""] # [doc = " The [`log!`], [`error!`], [`warn!`], [`info!`], [`debug!`], and [`trace!`] macros check"] # [doc = " this value and discard any message logged at a higher level. The maximum"] # [doc = " log level is set by the [`set_max_level`] function."] # [doc = ""] # [doc = " [`log!`]: macro.log.html"] # [doc = " [`error!`]: macro.error.html"] # [doc = " [`warn!`]: macro.warn.html"] # [doc = " [`info!`]: macro.info.html"] # [doc = " [`debug!`]: macro.debug.html"] # [doc = " [`trace!`]: macro.trace.html"] # [doc = " [`set_max_level`]: fn.set_max_level.html"] # [inline (always)] pub fn max_level () -> LevelFilter { unsafe { mem :: transmute (MAX_LOG_LEVEL_FILTER . load (Ordering :: Relaxed)) } }
};
}
