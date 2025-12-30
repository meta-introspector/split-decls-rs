// Generated macro for impl_84 (impl)
macro_rules! Depcrate_endpointimpl_84 {
() => {
// Module: crate::endpoint
// Provides: {"impl_84"}
// Dependencies: {}
impl Future for Accept < '_ > { type Output = Option < Incoming > ; fn poll (self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; let mut endpoint = this . endpoint . inner . state . lock () . unwrap () ; if endpoint . driver_lost { return Poll :: Ready (None) ; } if let Some (incoming) = endpoint . recv_state . incoming . pop_front () { drop (endpoint) ; let incoming = Incoming :: new (incoming , this . endpoint . inner . clone ()) ; return Poll :: Ready (Some (incoming)) ; } if endpoint . recv_state . connections . close . is_some () { return Poll :: Ready (None) ; } loop { match this . notify . as_mut () . poll (ctx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (()) => this . notify . set (this . endpoint . inner . shared . incoming . notified ()) , } } } }
};
}
