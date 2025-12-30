// Generated macro for impl_18 (impl)
macro_rules! Depcrate_connectionimpl_18 {
() => {
// Module: crate::connection
// Provides: {"impl_18"}
// Dependencies: {}
impl Future for ConnectionDriver { type Output = Result < () , io :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let conn = & mut * self . 0 . state . lock ("poll") ; let span = debug_span ! ("drive" , id = conn . handle . 0) ; let _guard = span . enter () ; if let Err (e) = conn . process_conn_events (& self . 0 . shared , cx) { conn . terminate (e , & self . 0 . shared) ; return Poll :: Ready (Ok (())) ; } let mut keep_going = conn . drive_transmit (cx) ? ; keep_going |= conn . drive_timer (cx) ; conn . forward_endpoint_events () ; conn . forward_app_events (& self . 0 . shared) ; if ! conn . inner . is_drained () { if keep_going { cx . waker () . wake_by_ref () ; } else { conn . driver = Some (cx . waker () . clone ()) ; } return Poll :: Pending ; } if conn . error . is_none () { unreachable ! ("drained connections always have an error") ; } Poll :: Ready (Ok (())) } }
};
}
