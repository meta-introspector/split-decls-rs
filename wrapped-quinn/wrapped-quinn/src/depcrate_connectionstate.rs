// Generated macro for State (struct)
macro_rules! Depcrate_connectionState {
() => {
// Module: crate::connection
// Provides: {"State"}
// Dependencies: {}
pub (crate) struct State { pub (crate) inner : proto :: Connection , driver : Option < Waker > , handle : ConnectionHandle , on_handshake_data : Option < oneshot :: Sender < () > > , on_connected : Option < oneshot :: Sender < bool > > , connected : bool , timer : Option < Pin < Box < dyn AsyncTimer > > > , timer_deadline : Option < Instant > , conn_events : mpsc :: UnboundedReceiver < ConnectionEvent > , endpoint_events : mpsc :: UnboundedSender < (ConnectionHandle , EndpointEvent) > , pub (crate) blocked_writers : FxHashMap < StreamId , Waker > , pub (crate) blocked_readers : FxHashMap < StreamId , Waker > , pub (crate) stopped : FxHashMap < StreamId , Arc < Notify > > , # [doc = " Always set to Some before the connection becomes drained"] pub (crate) error : Option < ConnectionError > , # [doc = " Number of live handles that can be used to initiate or handle I/O; excludes the driver"] ref_count : usize , sender : Pin < Box < dyn UdpSender > > , runtime : Arc < dyn Runtime > , send_buffer : Vec < u8 > , # [doc = " We buffer a transmit when the underlying I/O would block"] buffered_transmit : Option < proto :: Transmit > , }
};
}
