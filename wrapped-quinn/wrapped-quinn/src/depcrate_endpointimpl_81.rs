// Generated macro for impl_81 (impl)
macro_rules! Depcrate_endpointimpl_81 {
() => {
// Module: crate::endpoint
// Provides: {"impl_81"}
// Dependencies: {}
impl ConnectionSet { fn insert (& mut self , handle : ConnectionHandle , conn : proto :: Connection , sender : Pin < Box < dyn UdpSender > > , runtime : Arc < dyn Runtime > ,) -> Connecting { let (send , recv) = mpsc :: unbounded_channel () ; if let Some ((error_code , ref reason)) = self . close { send . send (ConnectionEvent :: Close { error_code , reason : reason . clone () , }) . unwrap () ; } self . senders . insert (handle , send) ; Connecting :: new (handle , conn , self . sender . clone () , recv , sender , runtime) } fn is_empty (& self) -> bool { self . senders . is_empty () } }
};
}
