// Generated macro for IN_EXCL_UNLINK (const)
macro_rules! DepcrateIN_EXCL_UNLINK {
() => {
// Module: crate
// Provides: {"IN_EXCL_UNLINK"}
// Dependencies: {}
# [doc = " Ignore events for children, that have been unlinked from watched directory"] # [doc = ""] # [doc = " This bit can be set in [`inotify_add_watch`]'s `mask` parameter, to"] # [doc = " configure the watch."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_EXCL_UNLINK : u32 = 0x04000000 ;
};
}
