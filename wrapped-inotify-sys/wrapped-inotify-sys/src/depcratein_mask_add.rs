// Generated macro for IN_MASK_ADD (const)
macro_rules! DepcrateIN_MASK_ADD {
() => {
// Module: crate
// Provides: {"IN_MASK_ADD"}
// Dependencies: {}
# [doc = " Update existing watch mask, instead of replacing it"] # [doc = ""] # [doc = " This bit can be set in [`inotify_add_watch`]'s `mask` parameter, to"] # [doc = " configure the watch."] # [doc = ""] # [doc = " See [man page] for additional details."] # [doc = ""] # [doc = " [`inotify_add_watch`]: fn.inotify_add_watch.html"] # [doc = " [man page]: http://man7.org/linux/man-pages/man7/inotify.7.html"] pub const IN_MASK_ADD : u32 = 0x20000000 ;
};
}
