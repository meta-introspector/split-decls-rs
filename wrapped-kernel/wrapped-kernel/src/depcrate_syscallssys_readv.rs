// Generated macro for sys_readv (function)
macro_rules! Depcrate_syscallssys_readv {
() => {
// Module: crate::syscalls
// Provides: {"sys_readv"}
// Dependencies: {}
# [doc = " `read()` attempts to read `nbyte` of data to the object referenced by the"] # [doc = " descriptor `fd` from a buffer. `read()` performs the same"] # [doc = " action, but scatters the input data from the `iovcnt` buffers specified by the"] # [doc = " members of the iov array: `iov[0], iov[1], ..., iov[iovcnt-1]`."] # [doc = ""] # [doc = " ```"] # [doc = " struct iovec {"] # [doc = "     char   *iov_base;  /* Base address. */"] # [doc = "     size_t iov_len;    /* Length. */"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Each `iovec` entry specifies the base address and length of an area in memory from"] # [doc = " which data should be written.  `readv()` will always fill an completely"] # [doc = " before proceeding to the next."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_readv (fd : i32 , iov : * const iovec , iovcnt : usize) -> isize { if ! (0 ..= IOV_MAX) . contains (& iovcnt) { return (- i32 :: from (Errno :: Inval)) . try_into () . unwrap () ; } let mut read_bytes : isize = 0 ; let iovec_buffers = unsafe { core :: slice :: from_raw_parts (iov , iovcnt) } ; for iovec_buf in iovec_buffers { let buf = unsafe { core :: slice :: from_raw_parts_mut (iovec_buf . iov_base . cast () , iovec_buf . iov_len) } ; let len = crate :: fd :: read (fd , buf) . map_or_else (| e | isize :: try_from (- i32 :: from (e)) . unwrap () , | v | v . try_into () . unwrap () ,) ; if len < 0 { return len ; } read_bytes += len ; if len < isize :: try_from (iovec_buf . iov_len) . unwrap () { return read_bytes ; } } read_bytes }
};
}
