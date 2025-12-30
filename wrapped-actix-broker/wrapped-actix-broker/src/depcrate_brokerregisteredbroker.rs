// Generated macro for RegisteredBroker (trait)
macro_rules! Depcrate_brokerRegisteredBroker {
() => {
// Module: crate::broker
// Provides: {"RegisteredBroker"}
// Dependencies: {}
pub trait RegisteredBroker : 'static + Unpin where Self : std :: marker :: Sized , { fn get_broker () -> Addr < Broker < Self > > ; }
};
}
