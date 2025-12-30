// Generated macro for impl_36 (impl)
macro_rules! Depcrate_connectionimpl_36 {
() => {
// Module: crate::connection
// Provides: {"impl_36"}
// Dependencies: {}
impl ConnectionRef { # [allow (clippy :: too_many_arguments)] fn new (handle : ConnectionHandle , conn : proto :: Connection , endpoint_events : mpsc :: UnboundedSender < (ConnectionHandle , EndpointEvent) > , conn_events : mpsc :: UnboundedReceiver < ConnectionEvent > , on_handshake_data : oneshot :: Sender < () > , on_connected : oneshot :: Sender < bool > , sender : Pin < Box < dyn UdpSender > > , runtime : Arc < dyn Runtime > ,) -> Self { Self (Arc :: new (ConnectionInner { state : Mutex :: new (State { inner : conn , driver : None , handle , on_handshake_data : Some (on_handshake_data) , on_connected : Some (on_connected) , connected : false , timer : None , timer_deadline : None , conn_events , endpoint_events , blocked_writers : FxHashMap :: default () , blocked_readers : FxHashMap :: default () , stopped : FxHashMap :: default () , error : None , ref_count : 0 , sender , runtime , send_buffer : Vec :: new () , buffered_transmit : None , }) , shared : Shared :: default () , })) } fn stable_id (& self) -> usize { & * self . 0 as * const _ as usize } }
};
}
