// Generated macro for set_max_level_racy (function)
macro_rules! Depcrateset_max_level_racy {
() => {
// Module: crate
// Provides: {"set_max_level_racy"}
// Dependencies: {}
# [doc = " A thread-unsafe version of [`set_max_level`]."] # [doc = ""] # [doc = " This function is available on all platforms, even those that do not have"] # [doc = " support for atomics that is needed by [`set_max_level`]."] # [doc = ""] # [doc = " In almost all cases, [`set_max_level`] should be preferred."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is only safe to call when it cannot race with any other"] # [doc = " calls to `set_max_level` or `set_max_level_racy`."] # [doc = ""] # [doc = " This can be upheld by (for example) making sure that **there are no other"] # [doc = " threads**, and (on embedded) that **interrupts are disabled**."] # [doc = ""] # [doc = " It is safe to use all other logging functions while this function runs"] # [doc = " (including all logging macros)."] # [doc = ""] # [doc = " [`set_max_level`]: fn.set_max_level.html"] # [inline] pub unsafe fn set_max_level_racy (level : LevelFilter) { MAX_LOG_LEVEL_FILTER . store (level as usize , Ordering :: Relaxed) ; }
};
}
