// Generated macro for State (struct)
macro_rules! Depcrate_endpointState {
() => {
// Module: crate::endpoint
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct State { socket : Box < dyn AsyncUdpSocket > , sender : Pin < Box < dyn UdpSender > > , # [doc = " During an active migration, abandoned_socket receives traffic"] # [doc = " until the first packet arrives on the new socket."] prev_socket : Option < Box < dyn AsyncUdpSocket > > , inner : proto :: Endpoint , recv_state : RecvState , driver : Option < Waker > , ipv6 : bool , events : mpsc :: UnboundedReceiver < (ConnectionHandle , EndpointEvent) > , # [doc = " Number of live handles that can be used to initiate or handle I/O; excludes the driver"] ref_count : usize , driver_lost : bool , runtime : Arc < dyn Runtime > , stats : EndpointStats , }
};
}
