// Generated macro for impl_86 (impl)
macro_rules! Depcrate_endpointimpl_86 {
() => {
// Module: crate::endpoint
// Provides: {"impl_86"}
// Dependencies: {}
impl EndpointRef { pub (crate) fn new (socket : Box < dyn AsyncUdpSocket > , inner : proto :: Endpoint , ipv6 : bool , runtime : Arc < dyn Runtime > ,) -> Self { let (sender , events) = mpsc :: unbounded_channel () ; let recv_state = RecvState :: new (sender , socket . max_receive_segments () , & inner) ; let sender = socket . create_sender () ; Self (Arc :: new (EndpointInner { shared : Shared { incoming : Notify :: new () , idle : Notify :: new () , } , state : Mutex :: new (State { socket , sender , prev_socket : None , inner , ipv6 , events , driver : None , ref_count : 0 , driver_lost : false , recv_state , runtime , stats : EndpointStats :: default () , }) , })) } }
};
}
