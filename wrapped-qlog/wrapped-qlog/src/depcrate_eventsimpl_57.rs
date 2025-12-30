// Generated macro for impl_57 (impl)
macro_rules! Depcrate_eventsimpl_57 {
() => {
// Module: crate::events
// Provides: {"impl_57"}
// Dependencies: {}
impl From < EventType > for EventCategory { fn from (ty : EventType) -> Self { match ty { EventType :: ConnectivityEventType (_) => EventCategory :: Connectivity , EventType :: SecurityEventType (_) => EventCategory :: Security , EventType :: TransportEventType (_) => EventCategory :: Transport , EventType :: RecoveryEventType (_) => EventCategory :: Recovery , EventType :: Http3EventType (_) => EventCategory :: Http , EventType :: QpackEventType (_) => EventCategory :: Qpack , _ => unimplemented ! () , } } }
};
}
