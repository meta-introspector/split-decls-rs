// Generated macro for TransportFactory (type)
macro_rules! Depcrate_transportTransportFactory {
() => {
// Module: crate::transport
// Provides: {"TransportFactory"}
// Dependencies: {}
type TransportFactory = dyn Fn (& Remote < '_ >) -> Result < Transport , Error > + Send + Sync + 'static ;
};
}
