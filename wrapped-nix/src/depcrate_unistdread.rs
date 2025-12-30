// Generated macro for read (function)
macro_rules! Depcrate_unistdread {
() => {
// Module: crate::unistd
// Provides: {"read"}
// Dependencies: {}
# [doc = " Read from a raw file descriptor."] # [doc = ""] # [doc = " See also [read(2)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/read.html)"] pub fn read < Fd : std :: os :: fd :: AsFd > (fd : Fd , buf : & mut [u8]) -> Result < usize > { use std :: os :: fd :: AsRawFd ; let res = unsafe { libc :: read (fd . as_fd () . as_raw_fd () , buf . as_mut_ptr () . cast () , buf . len () as size_t ,) } ; Errno :: result (res) . map (| r | r as usize) }
};
}
