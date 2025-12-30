// Generated macro for impl_70 (impl)
macro_rules! Depcrate_endpointimpl_70 {
() => {
// Module: crate::endpoint
// Provides: {"impl_70"}
// Dependencies: {}
impl Future for EndpointDriver { type Output = Result < () , io :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let mut endpoint = self . 0 . state . lock () . unwrap () ; if endpoint . driver . is_none () { endpoint . driver = Some (cx . waker () . clone ()) ; } let now = endpoint . runtime . now () ; let mut keep_going = false ; keep_going |= endpoint . drive_recv (cx , now) ? ; keep_going |= endpoint . handle_events (cx , & self . 0 . shared) ; if ! endpoint . recv_state . incoming . is_empty () { self . 0 . shared . incoming . notify_waiters () ; } if endpoint . ref_count == 0 && endpoint . recv_state . connections . is_empty () { Poll :: Ready (Ok (())) } else { drop (endpoint) ; if keep_going { cx . waker () . wake_by_ref () ; } Poll :: Pending } } }
};
}
