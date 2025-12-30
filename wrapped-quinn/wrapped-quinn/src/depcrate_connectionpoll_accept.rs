// Generated macro for poll_accept (function)
macro_rules! Depcrate_connectionpoll_accept {
() => {
// Module: crate::connection
// Provides: {"poll_accept"}
// Dependencies: {}
fn poll_accept < 'a > (ctx : & mut Context < '_ > , conn : & 'a ConnectionRef , mut notify : Pin < & mut Notified < 'a > > , dir : Dir ,) -> Poll < Result < (ConnectionRef , StreamId , bool) , ConnectionError > > { let mut state = conn . state . lock ("poll_accept") ; if let Some (id) = state . inner . streams () . accept (dir) { let is_0rtt = state . inner . is_handshaking () ; state . wake () ; drop (state) ; return Poll :: Ready (Ok ((conn . clone () , id , is_0rtt))) ; } else if let Some (ref e) = state . error { return Poll :: Ready (Err (e . clone ())) ; } loop { match notify . as_mut () . poll (ctx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (()) => notify . set (conn . shared . stream_incoming [dir as usize] . notified ()) , } } }
};
}
