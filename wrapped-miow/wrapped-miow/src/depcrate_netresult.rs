// Generated macro for result (function)
macro_rules! Depcrate_netresult {
() => {
// Module: crate::net
// Provides: {"result"}
// Dependencies: {}
unsafe fn result (socket : SOCKET , overlapped : * mut OVERLAPPED) -> io :: Result < (usize , u32) > { let mut transferred = 0 ; let mut flags = 0 ; let r = WSAGetOverlappedResult (socket , overlapped , & mut transferred , FALSE , & mut flags) ; if r == 0 { Err (io :: Error :: last_os_error ()) } else { Ok ((transferred as usize , flags)) } }
};
}
