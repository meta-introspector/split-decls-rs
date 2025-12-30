// Generated macro for last_err (function)
macro_rules! Depcrate_netlast_err {
() => {
// Module: crate::net
// Provides: {"last_err"}
// Dependencies: {}
fn last_err () -> io :: Result < Option < usize > > { let err = unsafe { WSAGetLastError () } ; if err == WSA_IO_PENDING { Ok (None) } else { Err (io :: Error :: from_raw_os_error (err)) } }
};
}
