// Generated macro for TestEndpoint (struct)
macro_rules! Depcrate_tests_utilTestEndpoint {
() => {
// Module: crate::tests::util
// Provides: {"TestEndpoint"}
// Dependencies: {}
pub (super) struct TestEndpoint { pub (super) endpoint : Endpoint , pub (super) addr : SocketAddr , socket : Option < UdpSocket > , timeout : Option < Instant > , pub (super) outbound : VecDeque < (Transmit , Bytes) > , delayed : VecDeque < (Transmit , Bytes) > , pub (super) inbound : VecDeque < (Instant , Option < EcnCodepoint > , BytesMut) > , accepted : Option < Result < ConnectionHandle , ConnectionError > > , pub (super) connections : HashMap < ConnectionHandle , Connection > , conn_events : HashMap < ConnectionHandle , VecDeque < ConnectionEvent > > , pub (super) captured_packets : Vec < Vec < u8 > > , pub (super) capture_inbound_packets : bool , pub (super) handle_incoming : Box < dyn FnMut (& Incoming) -> IncomingConnectionBehavior > , pub (super) waiting_incoming : Vec < Incoming > , }
};
}
