// Generated macro for GraphQLSubscription (struct)
macro_rules! Depcrate_subscriptionGraphQLSubscription {
() => {
// Module: crate::subscription
// Provides: {"GraphQLSubscription"}
// Dependencies: {}
# [doc = " A builder for websocket subscription actor."] pub struct GraphQLSubscription < E , OnInit , OnPing > { executor : E , data : Data , on_connection_init : OnInit , on_ping : OnPing , keepalive_timeout : Option < Duration > , }
};
}
