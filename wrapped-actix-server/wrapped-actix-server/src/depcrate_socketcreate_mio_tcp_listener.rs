// Generated macro for create_mio_tcp_listener (function)
macro_rules! Depcrate_socketcreate_mio_tcp_listener {
() => {
// Module: crate::socket
// Provides: {"create_mio_tcp_listener"}
// Dependencies: {}
pub (crate) fn create_mio_tcp_listener (addr : StdSocketAddr , backlog : u32 , mptcp : & MpTcp ,) -> io :: Result < MioTcpListener > { use socket2 :: { Domain , Protocol , Socket , Type } ; # [cfg (not (target_os = "linux"))] let protocol = Protocol :: TCP ; # [cfg (target_os = "linux")] let protocol = if matches ! (mptcp , MpTcp :: Disabled) { Protocol :: TCP } else { Protocol :: MPTCP } ; let socket = match Socket :: new (Domain :: for_address (addr) , Type :: STREAM , Some (protocol)) { Ok (sock) => sock , Err (err) if matches ! (mptcp , MpTcp :: TcpFallback) => { tracing :: warn ! ("binding socket as MPTCP failed: {err}") ; tracing :: warn ! ("falling back to TCP") ; Socket :: new (Domain :: for_address (addr) , Type :: STREAM , Some (Protocol :: TCP)) ? } Err (err) => return Err (err) , } ; socket . set_reuse_address (true) ? ; socket . set_nonblocking (true) ? ; socket . bind (& addr . into ()) ? ; socket . listen (backlog as i32) ? ; Ok (MioTcpListener :: from_std (StdTcpListener :: from (socket))) }
};
}
