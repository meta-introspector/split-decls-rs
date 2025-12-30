// Generated macro for bind_addr (function)
macro_rules! Depcrate_builderbind_addr {
() => {
// Module: crate::builder
// Provides: {"bind_addr"}
// Dependencies: {}
pub (super) fn bind_addr < S : ToSocketAddrs > (addr : S , backlog : u32 , mptcp : & MpTcp ,) -> io :: Result < Vec < MioTcpListener > > { let mut opt_err = None ; let mut success = false ; let mut sockets = Vec :: new () ; for addr in addr . to_socket_addrs () ? { match create_mio_tcp_listener (addr , backlog , mptcp) { Ok (lst) => { success = true ; sockets . push (lst) ; } Err (err) => opt_err = Some (err) , } } if success { Ok (sockets) } else if let Some (err) = opt_err . take () { Err (err) } else { Err (io :: Error :: other ("Can not bind to address.")) } }
};
}
