// Generated macro for IN_MOVE_SELF (const)
macro_rules! DepcrateIN_MOVE_SELF {
() => {
// Module: crate
// Provides: {"IN_MOVE_SELF"}
// Dependencies: {}
# [doc = " Event: Watched file or directory was moved"] # [doc = ""] # [doc = " This constant can be passed to [`inotify_add_watch`], to register interest"] # [doc = " in this type of event, or it can be used to check (via [`inotify_event`]'s"] # [doc = " [`mask`] field) whether an event is of this type."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_MOVE_SELF : u32 = 0x00000800 ;
};
}
