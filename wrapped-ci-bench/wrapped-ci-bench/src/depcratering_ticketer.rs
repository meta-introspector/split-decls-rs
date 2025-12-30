// Generated macro for ring_ticketer (function)
macro_rules! Depcratering_ticketer {
() => {
// Module: crate
// Provides: {"ring_ticketer"}
// Dependencies: {}
fn ring_ticketer () -> Arc < dyn TicketProducer > { ring :: DEFAULT_PROVIDER . ticketer_factory . ticketer () . unwrap () }
};
}
