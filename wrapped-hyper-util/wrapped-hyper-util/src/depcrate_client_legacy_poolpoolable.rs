// Generated macro for Poolable (trait)
macro_rules! Depcrate_client_legacy_poolPoolable {
() => {
// Module: crate::client::legacy::pool
// Provides: {"Poolable"}
// Dependencies: {}
pub trait Poolable : Unpin + Send + Sized + 'static { fn is_open (& self) -> bool ; # [doc = " Reserve this connection."] # [doc = ""] # [doc = " Allows for HTTP/2 to return a shared reservation."] fn reserve (self) -> Reservation < Self > ; fn can_share (& self) -> bool ; }
};
}
