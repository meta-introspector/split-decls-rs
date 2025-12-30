// Generated macro for impl_127 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_127 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_127"}
// Dependencies: {}
impl ConnectionCloseDetails { pub fn new (qconn : & Connection) -> Self { let session = qconn . session () . map (| s | s . to_vec ()) ; Self { peer_error : qconn . peer_error () . cloned () , local_error : qconn . local_error () . cloned () , timed_out : qconn . is_timed_out () , session , } } # [doc = " The error sent from the peer, if any."] pub fn peer_error (& self) -> Option < & ConnectionError > { self . peer_error . as_ref () } # [doc = " The error generated locally, if any."] pub fn local_error (& self) -> Option < & ConnectionError > { self . local_error . as_ref () } # [doc = " If the connection didn't see an error, either one from the peer or"] # [doc = " generated locally."] pub fn no_err (& self) -> bool { self . peer_error . is_none () && self . local_error . is_none () } }
};
}
