// Generated macro for unix_impl (module)
macro_rules! Depcrate_socketunix_impl {
() => {
// Module: crate::socket
// Provides: {"unix_impl"}
// Dependencies: {}
# [cfg (unix)] mod unix_impl { use std :: os :: unix :: io :: { FromRawFd , IntoRawFd } ; use actix_rt :: net :: UnixStream ; use super :: * ; impl FromStream for TcpStream { fn from_mio (sock : MioStream) -> io :: Result < Self > { match sock { MioStream :: Tcp (mio) => { let raw = IntoRawFd :: into_raw_fd (mio) ; TcpStream :: from_std (unsafe { FromRawFd :: from_raw_fd (raw) }) } MioStream :: Uds (_) => { panic ! ("Should not happen, bug in server impl") ; } } } } impl FromStream for UnixStream { fn from_mio (sock : MioStream) -> io :: Result < Self > { match sock { MioStream :: Tcp (_) => panic ! ("Should not happen, bug in server impl") , MioStream :: Uds (mio) => { let raw = IntoRawFd :: into_raw_fd (mio) ; UnixStream :: from_std (unsafe { FromRawFd :: from_raw_fd (raw) }) } } } } }
};
}
