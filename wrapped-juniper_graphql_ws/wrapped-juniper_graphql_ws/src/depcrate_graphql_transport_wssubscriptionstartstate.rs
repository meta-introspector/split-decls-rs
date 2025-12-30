// Generated macro for SubscriptionStartState (enum)
macro_rules! Depcrate_graphql_transport_wsSubscriptionStartState {
() => {
// Module: crate::graphql_transport_ws
// Provides: {"SubscriptionStartState"}
// Dependencies: {}
# [doc = " SubscriptionStartState is the state for a subscription operation."] enum SubscriptionStartState < S : Schema > { # [doc = " Init is the start before being polled for the first time."] Init { id : String } , # [doc = " ResolvingIntoStream is the state after being polled for the first time. In this state,"] # [doc = " we're parsing, validating, and getting the actual event stream."] ResolvingIntoStream { id : String , future : BoxFuture < 'static , Result < juniper_subscriptions :: Connection < 'static , S :: ScalarValue > , GraphQLError > , > , } , # [doc = " Streaming is the state after we've successfully obtained the event stream for the"] # [doc = " subscription. In this state, we're just forwarding events back to the client."] Streaming { id : String , stream : juniper_subscriptions :: Connection < 'static , S :: ScalarValue > , } , # [doc = " Terminated is the state once we're all done."] Terminated , }
};
}
