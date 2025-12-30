// Generated macro for impl_76 (impl)
macro_rules! Depcrate_endpointimpl_76 {
() => {
// Module: crate::endpoint
// Provides: {"impl_76"}
// Dependencies: {}
impl State { fn drive_recv (& mut self , cx : & mut Context , now : Instant) -> Result < bool , io :: Error > { let get_time = | | self . runtime . now () ; self . recv_state . recv_limiter . start_cycle (get_time) ; if let Some (socket) = & mut self . prev_socket { let poll_res = self . recv_state . poll_socket (cx , & mut self . inner , & mut * * socket , & mut self . sender , & * self . runtime , now ,) ; if poll_res . is_err () { self . prev_socket = None ; } } ; let poll_res = self . recv_state . poll_socket (cx , & mut self . inner , & mut * self . socket , & mut self . sender , & * self . runtime , now ,) ; self . recv_state . recv_limiter . finish_cycle (get_time) ; let poll_res = poll_res ? ; if poll_res . received_connection_packet { self . prev_socket = None ; } Ok (poll_res . keep_going) } fn handle_events (& mut self , cx : & mut Context , shared : & Shared) -> bool { for _ in 0 .. IO_LOOP_BOUND { let (ch , event) = match self . events . poll_recv (cx) { Poll :: Ready (Some (x)) => x , Poll :: Ready (None) => unreachable ! ("EndpointInner owns one sender") , Poll :: Pending => { return false ; } } ; if event . is_drained () { self . recv_state . connections . senders . remove (& ch) ; if self . recv_state . connections . is_empty () { shared . idle . notify_waiters () ; } } let Some (event) = self . inner . handle_event (ch , event) else { continue ; } ; let _ = self . recv_state . connections . senders . get_mut (& ch) . unwrap () . send (ConnectionEvent :: Proto (event)) ; } true } }
};
}
