// Generated macro for ServerEventMultiplexer (struct)
macro_rules! Depcrate_serverServerEventMultiplexer {
() => {
// Module: crate::server
// Provides: {"ServerEventMultiplexer"}
// Dependencies: {}
struct ServerEventMultiplexer { cmd_rx : UnboundedReceiver < ServerCommand > , signal_fut : Option < StopSignal > , }
};
}
