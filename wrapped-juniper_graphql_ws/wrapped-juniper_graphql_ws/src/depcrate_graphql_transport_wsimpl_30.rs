// Generated macro for impl_30 (impl)
macro_rules! Depcrate_graphql_transport_wsimpl_30 {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"impl_30"}
// Dependencies: {}
impl < S : Schema > SubscriptionStart < S > { fn new (id : String , params : Arc < ExecutionParams < S > >) -> Pin < Box < Self > > { Box :: pin (Self { params , state : SubscriptionStartState :: Init { id } , _marker : PhantomPinned , }) } }
};
}
