// Generated macro for IN_ISDIR (const)
macro_rules! DepcrateIN_ISDIR {
() => {
// Module: crate
// Provides: {"IN_ISDIR"}
// Dependencies: {}
# [doc = " Indicates that the subject of an event is a directory"] # [doc = ""] # [doc = " This constant can be used to check against the [`mask`] field in"] # [doc = " [`inotify_event`]."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_ISDIR : u32 = 0x40000000 ;
};
}
