// Generated macro for sys_writev (function)
macro_rules! Depcrate_syscallssys_writev {
() => {
// Module: crate::syscalls
// Provides: {"sys_writev"}
// Dependencies: {}
# [doc = " `write()` attempts to write `nbyte` of data to the object referenced by the"] # [doc = " descriptor `fd` from a buffer. `writev()` performs the same"] # [doc = " action, but gathers the output data from the `iovcnt` buffers specified by the"] # [doc = " members of the iov array: `iov[0], iov[1], ..., iov[iovcnt-1]`."] # [doc = ""] # [doc = " ```"] # [doc = " struct iovec {"] # [doc = "     char   *iov_base;  /* Base address. */"] # [doc = "     size_t iov_len;    /* Length. */"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Each `iovec` entry specifies the base address and length of an area in memory from"] # [doc = " which data should be written.  `writev()` will always write a"] # [doc = " complete area before proceeding to the next."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_writev (fd : FileDescriptor , iov : * const iovec , iovcnt : usize) -> isize { if ! (0 ..= IOV_MAX) . contains (& iovcnt) { return (- i32 :: from (Errno :: Inval)) . try_into () . unwrap () ; } let mut written_bytes : isize = 0 ; let iovec_buffers = unsafe { core :: slice :: from_raw_parts (iov , iovcnt) } ; for iovec_buf in iovec_buffers { let buf = unsafe { core :: slice :: from_raw_parts (iovec_buf . iov_base , iovec_buf . iov_len) } ; let len = crate :: fd :: write (fd , buf) . map_or_else (| e | isize :: try_from (- i32 :: from (e)) . unwrap () , | v | v . try_into () . unwrap () ,) ; if len < 0 { return len ; } written_bytes += len ; if len < isize :: try_from (iovec_buf . iov_len) . unwrap () { return written_bytes ; } } written_bytes }
};
}
