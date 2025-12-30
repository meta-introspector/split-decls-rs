// Generated macro for IN_ONESHOT (const)
macro_rules! DepcrateIN_ONESHOT {
() => {
// Module: crate
// Provides: {"IN_ONESHOT"}
// Dependencies: {}
# [doc = " Remove watch after one event"] # [doc = ""] # [doc = " This bit can be set in [`inotify_add_watch`]'s `mask` parameter, to"] # [doc = " configure the watch."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_ONESHOT : u32 = 0x80000000 ;
};
}
