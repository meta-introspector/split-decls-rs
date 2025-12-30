// Generated macro for sys_recvfrom (function)
macro_rules! Depcrate_syscallsys_recvfrom {
() => {
// Module: crate::syscall
// Provides: {"sys_recvfrom"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_recvfrom (_socket : i32 , _buf : * mut u8 , _len : usize , _flags : i32 , _addr : * mut abi :: sockaddr , _addrlen : * mut abi :: socklen_t ,) -> isize { - 22 }
};
}
