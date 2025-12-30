// Generated macro for close (function)
macro_rules! Depcrate_unistdclose {
() => {
// Module: crate::unistd
// Provides: {"close"}
// Dependencies: {}
# [doc = " Close a file descriptor."] # [doc = ""] # [doc = " If `fd` is an owned file descriptor, it is generally preferred to call"] # [doc = " `drop(fd)` rather than `close(fd)`."] pub fn close < Fd : std :: os :: fd :: IntoRawFd > (fd : Fd) -> Result < () > { let res = unsafe { libc :: close (fd . into_raw_fd ()) } ; Errno :: result (res) . map (drop) }
};
}
