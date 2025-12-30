// Generated macro for SubscriptionStart (struct)
macro_rules! Depcrate_graphql_wsSubscriptionStart {
() => {
// Module: crate::graphql_ws
// Provides: {"SubscriptionStart"}
// Dependencies: {}
# [doc = " SubscriptionStart is the stream for a subscription operation."] struct SubscriptionStart < S : Schema > { params : Arc < ExecutionParams < S > > , state : SubscriptionStartState < S > , _marker : PhantomPinned , }
};
}
