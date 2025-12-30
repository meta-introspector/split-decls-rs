// Generated macro for impl_65 (impl)
macro_rules! Depcrate_graphql_wsimpl_65 {
() => {
// Module: crate::graphql_ws
// Provides: {"impl_65"}
// Dependencies: {}
impl < S : Schema > SubscriptionStart < S > { fn new (id : String , params : Arc < ExecutionParams < S > >) -> Pin < Box < Self > > { Box :: pin (Self { params , state : SubscriptionStartState :: Init { id } , _marker : PhantomPinned , }) } }
};
}
