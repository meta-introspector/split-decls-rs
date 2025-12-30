// Generated macro for IN_IGNORED (const)
macro_rules! DepcrateIN_IGNORED {
() => {
// Module: crate
// Provides: {"IN_IGNORED"}
// Dependencies: {}
# [doc = " Indicates that a file system watch was removed"] # [doc = ""] # [doc = " This can occur as a result of [`inotify_rm_watch`], because a watched item"] # [doc = "  was deleted, the containing filesystem was unmounted, or after a"] # [doc = " [`IN_ONESHOT`] watch is complete."] # [doc = ""] # [doc = " This constant can be used to check against the [`mask`] field in"] # [doc = " [`inotify_event`]."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_rm_watch`]: fn.inotify_rm_watch.html"] # [doc = " [`IN_ONESHOT`]: constant.IN_ONESHOT.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_IGNORED : u32 = 0x00008000 ;
};
}
