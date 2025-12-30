// Generated macro for IN_DELETE_SELF (const)
macro_rules! DepcrateIN_DELETE_SELF {
() => {
// Module: crate
// Provides: {"IN_DELETE_SELF"}
// Dependencies: {}
# [doc = " Event: Watched file or directory was deleted"] # [doc = ""] # [doc = " This may also occur if the object is moved to another filesystem, since"] # [doc = " [mv(1)] in effect copies the file to the other filesystem and then deletes"] # [doc = " it from the original."] # [doc = ""] # [doc = " An IN_IGNORED event will subsequently be generated."] # [doc = ""] # [doc = " This constant can be passed to [`inotify_add_watch`], to register interest"] # [doc = " in this type of event, or it can be used to check (via [`inotify_event`]'s"] # [doc = " [`mask`] field) whether an event is of this type."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [mv(1)]: http://man7.org/linux/man-pages/man1/mv.1.html"] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [`inotify_event`]: struct.inotify_event.html"] # [doc = " [`mask`]: struct.inotify_event.html#structfield.mask"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_DELETE_SELF : u32 = 0x00000400 ;
};
}
