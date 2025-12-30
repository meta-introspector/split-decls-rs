// Generated macro for base_socket (function)
macro_rules! Depcrate_os_iocp_afdbase_socket {
() => {
// Module: crate::os::iocp::afd
// Provides: {"base_socket"}
// Dependencies: {}
# [doc = " Get the base socket associated with a socket."] pub (super) fn base_socket (sock : RawSocket) -> io :: Result < RawSocket > { let result = unsafe { try_socket_ioctl (sock , SIO_BASE_HANDLE) } ; match result { Ok (sock) => return Ok (sock) , Err (e) if e . kind () == io :: ErrorKind :: InvalidInput => return Err (e) , Err (_) => { } } let result = unsafe { try_socket_ioctl (sock , SIO_BSP_HANDLE_POLL) ? } ; if result == sock { return Err (io :: Error :: from (io :: ErrorKind :: InvalidInput)) ; } unsafe { try_socket_ioctl (result , SIO_BASE_HANDLE) } }
};
}
