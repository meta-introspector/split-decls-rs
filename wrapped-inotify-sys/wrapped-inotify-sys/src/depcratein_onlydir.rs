// Generated macro for IN_ONLYDIR (const)
macro_rules! DepcrateIN_ONLYDIR {
() => {
// Module: crate
// Provides: {"IN_ONLYDIR"}
// Dependencies: {}
# [doc = " Only watch path, if it is a directory"] # [doc = ""] # [doc = " This bit can be set in [`inotify_add_watch`]'s `mask` parameter, to"] # [doc = " configure the watch."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_ONLYDIR : u32 = 0x01000000 ;
};
}
