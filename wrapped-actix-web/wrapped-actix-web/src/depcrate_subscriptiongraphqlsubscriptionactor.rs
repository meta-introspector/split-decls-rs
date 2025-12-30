// Generated macro for GraphQLSubscriptionActor (struct)
macro_rules! Depcrate_subscriptionGraphQLSubscriptionActor {
() => {
// Module: crate::subscription
// Provides: {"GraphQLSubscriptionActor"}
// Dependencies: {}
struct GraphQLSubscriptionActor < E , OnInit , OnPing > { executor : E , data : Option < Data > , protocol : WebSocketProtocols , last_heartbeat : Instant , messages : Option < async_channel :: Sender < Vec < u8 > > > , on_connection_init : Option < OnInit > , on_ping : OnPing , keepalive_timeout : Option < Duration > , continuation : Vec < u8 > , }
};
}
