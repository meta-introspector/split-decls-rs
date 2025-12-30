// Generated macro for IN_MOVE (const)
macro_rules! DepcrateIN_MOVE {
() => {
// Module: crate
// Provides: {"IN_MOVE"}
// Dependencies: {}
# [doc = " Event: File or directory within watched directory was moved"] # [doc = ""] # [doc = " This is a combination of [`IN_MOVED_FROM`] and [`IN_MOVED_TO`]."] # [doc = ""] # [doc = " This constant can be passed to [`inotify_add_watch`], to register interest"] # [doc = " in this type of event, or it can be used to check (via [`inotify_event`]'s"] # [doc = " [`mask`] field) whether an event is of this type."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`IN_MOVED_FROM`]: constant.IN_MOVED_FROM.html"] # [doc = " [`IN_MOVED_TO`]: constant.IN_MOVED_TO.html"] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_MOVE : u32 = IN_MOVED_FROM | IN_MOVED_TO ;
};
}
