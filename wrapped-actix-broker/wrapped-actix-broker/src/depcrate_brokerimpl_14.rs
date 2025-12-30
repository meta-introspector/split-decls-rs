// Generated macro for impl_14 (impl)
macro_rules! Depcrate_brokerimpl_14 {
() => {
// Module: crate::broker
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : 'static + Unpin , M : BrokerMsg > Handler < SubscribeAsync < M > > for Broker < T > { type Result = () ; fn handle (& mut self , msg : SubscribeAsync < M > , _ctx : & mut Context < Self >) { trace ! ("Broker: Received SubscribeAsync") ; self . add_sub :: < M > (msg . 0 , msg . 1) ; } }
};
}
