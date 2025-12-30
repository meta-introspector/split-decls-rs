// Generated macro for SubscriptionConnection (trait)
macro_rules! Depcrate_types_subscriptionsSubscriptionConnection {
() => {
// Module: crate::types::subscriptions
// Provides: {"SubscriptionConnection"}
// Dependencies: {}
# [doc = " Single subscription connection."] # [doc = ""] # [doc = " This trait implementation might:"] # [doc = " - hold schema + context"] # [doc = " - process subscribe, unsubscribe"] # [doc = " - unregister from coordinator upon close/shutdown"] # [doc = " - connection-local + global de-duplication, talk to coordinator"] # [doc = " - concurrency limits"] # [doc = " - machinery with coordinator to allow reconnection"] # [doc = ""] # [doc = " It can be treated as [`futures::Stream`] yielding [`GraphQLResponse`]s in"] # [doc = " server integration crates."] # [doc = ""] # [doc = " [`GraphQLResponse`]: crate::http::GraphQLResponse"] pub trait SubscriptionConnection < S > : futures :: Stream < Item = ExecutionOutput < S > > { }
};
}
