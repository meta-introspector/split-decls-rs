// Generated macro for Broker (struct)
macro_rules! Depcrate_brokerBroker {
() => {
// Module: crate::broker
// Provides: {"Broker"}
// Dependencies: {}
# [derive (Default)] pub struct Broker < T > { sub_map : TypeMap < Vec < (TypeId , Box < dyn Any >) > > , msg_map : TypeMap < Box < dyn Any > > , _t : PhantomData < T > , }
};
}
