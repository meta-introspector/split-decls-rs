// Generated macro for AT_FDCWD (const)
macro_rules! Depcrate_fcntlAT_FDCWD {
() => {
// Module: crate::fcntl
// Provides: {"AT_FDCWD"}
// Dependencies: {}
# [doc = " A file descriptor referring to the working directory of the current process"] # [doc = " **that should be ONLY passed to the `dirfd` argument of those `xxat()` functions**."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Use it in [`openat()`]:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use nix::fcntl::AT_FDCWD;"] # [doc = " use nix::fcntl::openat;"] # [doc = " use nix::fcntl::OFlag;"] # [doc = " use nix::sys::stat::Mode;"] # [doc = ""] # [doc = " let fd = openat(AT_FDCWD, \"foo\", OFlag::O_RDONLY | OFlag::O_CLOEXEC, Mode::empty()).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " # WARNING"] # [doc = ""] # [doc = " Do NOT pass this symbol to non-`xxat()` functions, it won't work:"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use nix::errno::Errno;"] # [doc = " use nix::fcntl::AT_FDCWD;"] # [doc = " use nix::sys::stat::fstat;"] # [doc = ""] # [doc = " let never = fstat(AT_FDCWD).unwrap();"] # [doc = " ```"] # [cfg (not (target_os = "redox"))] pub const AT_FDCWD : std :: os :: fd :: BorrowedFd < 'static > = unsafe { std :: os :: fd :: BorrowedFd :: borrow_raw (libc :: AT_FDCWD) } ;
};
}
