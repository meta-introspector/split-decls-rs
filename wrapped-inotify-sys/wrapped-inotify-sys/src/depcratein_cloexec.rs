// Generated macro for IN_CLOEXEC (const)
macro_rules! DepcrateIN_CLOEXEC {
() => {
// Module: crate
// Provides: {"IN_CLOEXEC"}
// Dependencies: {}
# [doc = " Set the `FD_CLOEXEC` flag for an inotify instance"] # [doc = ""] # [doc = " Can be passed to [`inotify_init1`] to set the `FD_CLOEXEC` flag for the"] # [doc = " inotify instance. This changes the behavior of file descriptor when"] # [doc = " [execve(2)]'d. From [fcntl(2)]:"] # [doc = ""] # [doc = " > If the FD_CLOEXEC bit is 0, the file descriptor will"] # [doc = " > remain open across an [execve(2)], otherwise it will be"] # [doc = " > closed."] # [doc = ""] # [doc = " See [open(2)] and [fcntl(2)] for details."] # [doc = ""] # [doc = " [`inotify_init1`]: fn.inotify_init1.html"] # [doc = " [execve(2)]: http://man7.org/linux/man-pages/man2/execve.2.html"] # [doc = " [open(2)]: http://man7.org/linux/man-pages/man2/open.2.html"] # [doc = " [fcntl(2)]: http://man7.org/linux/man-pages/man2/fcntl.2.html"] pub const IN_CLOEXEC : c_int = libc :: O_CLOEXEC ;
};
}
