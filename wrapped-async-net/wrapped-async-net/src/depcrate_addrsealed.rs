// Generated macro for Sealed (trait)
macro_rules! Depcrate_addrSealed {
() => {
// Module: crate::addr
// Provides: {"Sealed"}
// Dependencies: {}
pub trait Sealed { # [doc = " Returned iterator over socket addresses which this type may correspond to."] type Iter : Iterator < Item = SocketAddr > + Unpin ; # [doc = " Converts this object to an iterator of resolved `SocketAddr`s."] # [doc = ""] # [doc = " The returned iterator may not actually yield any values depending on the outcome of any"] # [doc = " resolution performed."] # [doc = ""] # [doc = " Note that this function may block a backend thread while resolution is performed."] fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > ; }
};
}
