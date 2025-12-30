// Generated macro for IN_NONBLOCK (const)
macro_rules! DepcrateIN_NONBLOCK {
() => {
// Module: crate
// Provides: {"IN_NONBLOCK"}
// Dependencies: {}
# [doc = " Set an inotify instance to non-blocking mode"] # [doc = ""] # [doc = " Can be passed to [`inotify_init1`] to set the `O_NONBLOCK` flag for the"] # [doc = " inotify instance."] # [doc = ""] # [doc = " See [open(2)] for details."] # [doc = ""] # [doc = " [`inotify_init1`]: fn.inotify_init1.html"] # [doc = " [open(2)]: http://man7.org/linux/man-pages/man2/open.2.html"] pub const IN_NONBLOCK : c_int = libc :: O_NONBLOCK ;
};
}
