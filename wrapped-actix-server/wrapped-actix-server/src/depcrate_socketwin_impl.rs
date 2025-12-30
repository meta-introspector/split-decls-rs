// Generated macro for win_impl (module)
macro_rules! Depcrate_socketwin_impl {
() => {
// Module: crate::socket
// Provides: {"win_impl"}
// Dependencies: {}
# [cfg (windows)] mod win_impl { use std :: os :: windows :: io :: { FromRawSocket , IntoRawSocket } ; use super :: * ; impl FromStream for TcpStream { fn from_mio (sock : MioStream) -> io :: Result < Self > { match sock { MioStream :: Tcp (mio) => { let raw = IntoRawSocket :: into_raw_socket (mio) ; TcpStream :: from_std (unsafe { FromRawSocket :: from_raw_socket (raw) }) } } } } }
};
}
