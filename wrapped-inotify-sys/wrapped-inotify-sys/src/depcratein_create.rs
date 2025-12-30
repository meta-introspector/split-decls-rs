// Generated macro for IN_CREATE (const)
macro_rules! DepcrateIN_CREATE {
() => {
// Module: crate
// Provides: {"IN_CREATE"}
// Dependencies: {}
# [doc = " Event: File or directory was created in watched directory"] # [doc = ""] # [doc = " This may also include hard links, symlinks, and UNIX sockets."] # [doc = ""] # [doc = " This constant can be passed to [`inotify_add_watch`], to register interest"] # [doc = " in this type of event, or it can be used to check (via [`inotify_event`]'s"] # [doc = " [`mask`] field) whether an event is of this type."] # [doc = ""] # [doc = " When monitoring a directory, this event will be triggered only for files"] # [doc = " within the directory."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_CREATE : u32 = 0x00000100 ;
};
}
