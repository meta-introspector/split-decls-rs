// Generated macro for impl_32 (impl)
macro_rules! Depcrate_connectionimpl_32 {
() => {
// Module: crate::connection
// Provides: {"impl_32"}
// Dependencies: {}
impl Future for ReadDatagram < '_ > { type Output = Result < Bytes , ConnectionError > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; let mut state = this . conn . state . lock ("ReadDatagram::poll") ; if let Some (x) = state . inner . datagrams () . recv () { return Poll :: Ready (Ok (x)) ; } else if let Some (ref e) = state . error { return Poll :: Ready (Err (e . clone ())) ; } loop { match this . notify . as_mut () . poll (ctx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (()) => this . notify . set (this . conn . shared . datagram_received . notified ()) , } } } }
};
}
