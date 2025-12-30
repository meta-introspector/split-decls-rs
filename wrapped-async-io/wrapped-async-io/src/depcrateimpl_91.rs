// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl Stream for Timer { type Item = Instant ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . get_mut () ; if let Some (ref mut when) = this . when { if Instant :: now () >= * when { if let Some ((id , _)) = this . id_and_waker . take () { Reactor :: get () . remove_timer (* when , id) ; } let result_time = * when ; if let Some (next) = (* when) . checked_add (this . period) { * when = next ; let id = Reactor :: get () . insert_timer (next , cx . waker ()) ; this . id_and_waker = Some ((id , cx . waker () . clone ())) ; } else { this . when = None ; } return Poll :: Ready (Some (result_time)) ; } else { match & this . id_and_waker { None => { let id = Reactor :: get () . insert_timer (* when , cx . waker ()) ; this . id_and_waker = Some ((id , cx . waker () . clone ())) ; } Some ((id , w)) if ! w . will_wake (cx . waker ()) => { Reactor :: get () . remove_timer (* when , * id) ; let id = Reactor :: get () . insert_timer (* when , cx . waker ()) ; this . id_and_waker = Some ((id , cx . waker () . clone ())) ; } Some (_) => { } } } } Poll :: Pending } }
};
}
