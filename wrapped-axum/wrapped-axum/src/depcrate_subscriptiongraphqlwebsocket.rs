// Generated macro for GraphQLWebSocket (struct)
macro_rules! Depcrate_subscriptionGraphQLWebSocket {
() => {
// Module: crate::subscription
// Provides: {"GraphQLWebSocket"}
// Dependencies: {}
# [doc = " A Websocket connection for GraphQL subscription."] pub struct GraphQLWebSocket < Sink , Stream , E , OnConnInit , OnPing > { sink : Sink , stream : Stream , executor : E , data : Data , on_connection_init : OnConnInit , on_ping : OnPing , protocol : GraphQLProtocol , keepalive_timeout : Option < Duration > , }
};
}
