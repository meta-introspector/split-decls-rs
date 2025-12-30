// Generated macro for impl_15 (impl)
macro_rules! Depcrate_brokerimpl_15 {
() => {
// Module: crate::broker
// Provides: {"impl_15"}
// Dependencies: {}
impl < T : 'static + Unpin , M : BrokerMsg > Handler < SubscribeSync < M > > for Broker < T > { type Result = Option < M > ; fn handle (& mut self , msg : SubscribeSync < M > , _ctx : & mut Context < Self >) -> Self :: Result { trace ! ("Broker: Received SubscribeSync") ; self . add_sub :: < M > (msg . 0 , msg . 1) ; self . get_previous_msg :: < M > () } }
};
}
