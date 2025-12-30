// Generated macro for poll_open (function)
macro_rules! Depcrate_connectionpoll_open {
() => {
// Module: crate::connection
// Provides: {"poll_open"}
// Dependencies: {}
fn poll_open < 'a > (ctx : & mut Context < '_ > , conn : & 'a ConnectionRef , mut notify : Pin < & mut Notified < 'a > > , dir : Dir ,) -> Poll < Result < (ConnectionRef , StreamId , bool) , ConnectionError > > { let mut state = conn . state . lock ("poll_open") ; if let Some (ref e) = state . error { return Poll :: Ready (Err (e . clone ())) ; } else if let Some (id) = state . inner . streams () . open (dir) { let is_0rtt = state . inner . side () . is_client () && state . inner . is_handshaking () ; drop (state) ; return Poll :: Ready (Ok ((conn . clone () , id , is_0rtt))) ; } loop { match notify . as_mut () . poll (ctx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (()) => { notify . set (conn . shared . stream_budget_available [dir as usize] . notified ()) } } } }
};
}
