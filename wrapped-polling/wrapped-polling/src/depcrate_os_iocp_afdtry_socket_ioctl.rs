// Generated macro for try_socket_ioctl (function)
macro_rules! Depcrate_os_iocp_afdtry_socket_ioctl {
() => {
// Module: crate::os::iocp::afd
// Provides: {"try_socket_ioctl"}
// Dependencies: {}
# [doc = " Run an IOCTL on a socket and return a socket."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `ioctl` parameter must be a valid I/O control that returns a valid socket."] unsafe fn try_socket_ioctl (sock : RawSocket , ioctl : u32) -> io :: Result < RawSocket > { let mut out = MaybeUninit :: < RawSocket > :: uninit () ; let mut bytes = 0u32 ; let result = WSAIoctl (sock as _ , ioctl , ptr :: null_mut () , 0 , out . as_mut_ptr () . cast () , size_of :: < RawSocket > () as u32 , & mut bytes , ptr :: null_mut () , None ,) ; if result == SOCKET_ERROR { return Err (io :: Error :: last_os_error ()) ; } Ok (out . assume_init ()) }
};
}
