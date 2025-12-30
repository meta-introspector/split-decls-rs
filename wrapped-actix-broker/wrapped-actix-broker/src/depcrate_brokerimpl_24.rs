// Generated macro for impl_24 (impl)
macro_rules! Depcrate_brokerimpl_24 {
() => {
// Module: crate::broker
// Provides: {"impl_24"}
// Dependencies: {}
impl RegisteredBroker for SystemBroker { fn get_broker () -> Addr < Broker < Self > > { Broker :: < SystemBroker > :: from_registry () } }
};
}
