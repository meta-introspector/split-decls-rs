// Generated macro for Incoming (struct)
macro_rules! Depcrate_unixIncoming {
() => {
// Module: crate::unix
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " A stream of incoming Unix connections."] # [doc = ""] # [doc = " This stream is infinite, i.e awaiting the next connection will never result in [`None`]. It is"] # [doc = " created by the [`UnixListener::incoming()`] method."] pub struct Incoming < 'a > { incoming : Pin < Box < dyn Stream < Item = io :: Result < Async < std :: os :: unix :: net :: UnixStream > > > + Send + Sync + 'a , > , > , }
};
}
