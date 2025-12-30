// Generated macro for Incoming (struct)
macro_rules! Depcrate_tcpIncoming {
() => {
// Module: crate::tcp
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " A stream of incoming TCP connections."] # [doc = ""] # [doc = " This stream is infinite, i.e awaiting the next connection will never result in [`None`]. It is"] # [doc = " created by the [`TcpListener::incoming()`] method."] pub struct Incoming < 'a > { incoming : Pin < Box < dyn Stream < Item = io :: Result < Async < std :: net :: TcpStream > > > + Send + Sync + 'a > > , }
};
}
