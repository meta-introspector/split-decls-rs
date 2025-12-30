// Generated macro for impl_603 (impl)
macro_rules! Depcrate_connectionimpl_603 {
() => {
// Module: crate::connection
// Provides: {"impl_603"}
// Dependencies: {}
impl State { fn closed < R : Into < Close > > (reason : R) -> Self { Self :: Closed (state :: Closed { reason : reason . into () , }) } fn is_handshake (& self) -> bool { matches ! (* self , Self :: Handshake (_)) } fn is_established (& self) -> bool { matches ! (* self , Self :: Established) } fn is_closed (& self) -> bool { matches ! (* self , Self :: Closed (_) | Self :: Draining | Self :: Drained) } fn is_drained (& self) -> bool { matches ! (* self , Self :: Drained) } }
};
}
