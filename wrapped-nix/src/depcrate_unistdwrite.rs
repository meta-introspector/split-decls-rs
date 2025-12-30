// Generated macro for write (function)
macro_rules! Depcrate_unistdwrite {
() => {
// Module: crate::unistd
// Provides: {"write"}
// Dependencies: {}
# [doc = " Write to a raw file descriptor."] # [doc = ""] # [doc = " See also [write(2)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/write.html)"] pub fn write < Fd : std :: os :: fd :: AsFd > (fd : Fd , buf : & [u8]) -> Result < usize > { use std :: os :: fd :: AsRawFd ; let res = unsafe { libc :: write (fd . as_fd () . as_raw_fd () , buf . as_ptr () . cast () , buf . len () as size_t ,) } ; Errno :: result (res) . map (| r | r as usize) }
};
}
