// Generated macro for impl_34 (impl)
macro_rules! Depcrate_connectionimpl_34 {
() => {
// Module: crate::connection
// Provides: {"impl_34"}
// Dependencies: {}
impl Future for SendDatagram < '_ > { type Output = Result < () , SendDatagramError > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; let mut state = this . conn . state . lock ("SendDatagram::poll") ; if let Some (ref e) = state . error { return Poll :: Ready (Err (SendDatagramError :: ConnectionLost (e . clone ()))) ; } use proto :: SendDatagramError :: * ; match state . inner . datagrams () . send (this . data . take () . unwrap () , false) { Ok (()) => { state . wake () ; Poll :: Ready (Ok (())) } Err (e) => Poll :: Ready (Err (match e { Blocked (data) => { this . data . replace (data) ; loop { match this . notify . as_mut () . poll (ctx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (()) => this . notify . set (this . conn . shared . datagrams_unblocked . notified ()) , } } } UnsupportedByPeer => SendDatagramError :: UnsupportedByPeer , Disabled => SendDatagramError :: Disabled , TooLarge => SendDatagramError :: TooLarge , })) , } } }
};
}
