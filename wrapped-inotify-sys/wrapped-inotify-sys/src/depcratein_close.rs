// Generated macro for IN_CLOSE (const)
macro_rules! DepcrateIN_CLOSE {
() => {
// Module: crate
// Provides: {"IN_CLOSE"}
// Dependencies: {}
# [doc = " Event: File was closed"] # [doc = ""] # [doc = " This is a combination of [`IN_CLOSE_WRITE`] and [`IN_CLOSE_NOWRITE`]."] # [doc = ""] # [doc = " This constant can be passed to [`inotify_add_watch`], to register interest"] # [doc = " in this type of event, or it can be used to check (via [`inotify_event`]'s"] # [doc = " [`mask`] field) whether an event is of this type."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`IN_CLOSE_WRITE`]: constant.IN_CLOSE_WRITE.html"] # [doc = " [`IN_CLOSE_NOWRITE`]: constant.IN_CLOSE_NOWRITE.html"] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_CLOSE : u32 = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE ;
};
}
