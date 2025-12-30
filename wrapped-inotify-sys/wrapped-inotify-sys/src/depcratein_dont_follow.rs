// Generated macro for IN_DONT_FOLLOW (const)
macro_rules! DepcrateIN_DONT_FOLLOW {
() => {
// Module: crate
// Provides: {"IN_DONT_FOLLOW"}
// Dependencies: {}
# [doc = " Don't dereference path, if it is a symbolic link"] # [doc = ""] # [doc = " This bit can be set in [`inotify_add_watch`]'s `mask` parameter, to"] # [doc = " configure the watch."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_DONT_FOLLOW : u32 = 0x02000000 ;
};
}
