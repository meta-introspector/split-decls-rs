// Generated macro for Runtime (trait)
macro_rules! Depcrate_runtimeRuntime {
() => {
// Module: crate::runtime
// Provides: {"Runtime"}
// Dependencies: {}
# [doc = " Abstracts I/O and timer operations for runtime independence"] pub trait Runtime : Send + Sync + Debug + 'static { # [doc = " Construct a timer that will expire at `i`"] fn new_timer (& self , i : Instant) -> Pin < Box < dyn AsyncTimer > > ; # [doc = " Drive `future` to completion in the background"] # [track_caller] fn spawn (& self , future : Pin < Box < dyn Future < Output = () > + Send > >) ; # [doc = " Convert `t` into the socket type used by this runtime"] # [cfg (not (wasm_browser))] fn wrap_udp_socket (& self , t : std :: net :: UdpSocket) -> io :: Result < Box < dyn AsyncUdpSocket > > ; # [doc = " Look up the current time"] # [doc = ""] # [doc = " Allows simulating the flow of time for testing."] fn now (& self) -> Instant { Instant :: now () } }
};
}
