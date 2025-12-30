// Generated macro for iovec (struct)
macro_rules! Depcrate_syscallsiovec {
() => {
// Module: crate::syscalls
// Provides: {"iovec"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] # [doc = " Describes  a  region  of  memory, beginning at `iov_base` address and with the size of `iov_len` bytes."] struct iovec { # [doc = " Starting address"] pub iov_base : * mut u8 , # [doc = " Size of the memory pointed to by iov_base."] pub iov_len : usize , }
};
}
