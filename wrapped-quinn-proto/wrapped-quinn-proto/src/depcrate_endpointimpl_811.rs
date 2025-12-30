// Generated macro for impl_811 (impl)
macro_rules! Depcrate_endpointimpl_811 {
() => {
// Module: crate::endpoint
// Provides: {"impl_811"}
// Dependencies: {}
impl ResetTokenTable { fn insert (& mut self , remote : SocketAddr , token : ResetToken , ch : ConnectionHandle) -> bool { self . 0 . entry (remote) . or_default () . insert (token , ch) . is_some () } fn remove (& mut self , remote : SocketAddr , token : ResetToken) { use std :: collections :: hash_map :: Entry ; match self . 0 . entry (remote) { Entry :: Vacant (_) => { } Entry :: Occupied (mut e) => { e . get_mut () . remove (& token) ; if e . get () . is_empty () { e . remove_entry () ; } } } } fn get (& self , remote : SocketAddr , token : & [u8]) -> Option < & ConnectionHandle > { let token = ResetToken :: from (< [u8 ; RESET_TOKEN_SIZE] > :: try_from (token) . ok () ?) ; self . 0 . get (& remote) ? . get (& token) } }
};
}
