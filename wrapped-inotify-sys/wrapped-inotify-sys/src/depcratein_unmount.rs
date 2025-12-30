// Generated macro for IN_UNMOUNT (const)
macro_rules! DepcrateIN_UNMOUNT {
() => {
// Module: crate
// Provides: {"IN_UNMOUNT"}
// Dependencies: {}
# [doc = " Indicates that file system containing a watched object has been unmounted"] # [doc = ""] # [doc = " An [`IN_IGNORED`] event will be generated subsequently."] # [doc = ""] # [doc = " This constant can be used to check against the [`mask`] field in"] # [doc = " [`inotify_event`]."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`IN_IGNORED`]: constant.IN_IGNORED.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_UNMOUNT : u32 = 0x00002000 ;
};
}
