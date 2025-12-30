// Generated macro for IN_ATTRIB (const)
macro_rules! DepcrateIN_ATTRIB {
() => {
// Module: crate
// Provides: {"IN_ATTRIB"}
// Dependencies: {}
# [doc = " Event: Metadata was changed"] # [doc = ""] # [doc = " This can include e.g."] # [doc = ""] # [doc = " - permissions, see [chmod(2)];"] # [doc = " - timestamps, see [utimensat(2)];"] # [doc = " - extended attributes, see [setxattr(2)];"] # [doc = " - link count, see [link(2)] and [unlink(2)];"] # [doc = " - user/group, see [chown(2)]."] # [doc = ""] # [doc = " This constant can be passed to [`inotify_add_watch`], to register interest"] # [doc = " in this type of event, or it can be used to check (via [`inotify_event`]'s"] # [doc = " [`mask`] field) whether an event is of this type."] # [doc = ""] # [doc = " When monitoring a directory, this event can be triggered for both for the"] # [doc = " directory itself and the files within."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [chmod(2)]: http://man7.org/linux/man-pages/man2/chmod.2.html"] # [doc = " [utimensat(2)]: http://man7.org/linux/man-pages/man2/utimensat.2.html"] # [doc = " [setxattr(2)]: http://man7.org/linux/man-pages/man2/fsetxattr.2.html"] # [doc = " [link(2)]: http://man7.org/linux/man-pages/man2/link.2.html"] # [doc = " [unlink(2)]: http://man7.org/linux/man-pages/man2/unlink.2.html"] # [doc = " [chown(2)]: http://man7.org/linux/man-pages/man2/chown.2.html"] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_ATTRIB : u32 = 0x00000004 ;
};
}
