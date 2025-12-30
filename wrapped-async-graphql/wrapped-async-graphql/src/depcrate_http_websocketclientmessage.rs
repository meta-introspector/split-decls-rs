// Generated macro for ClientMessage (enum)
macro_rules! Depcrate_http_websocketClientMessage {
() => {
// Module: crate::http::websocket
// Provides: {"ClientMessage"}
// Dependencies: {}
# [doc = " A websocket message received from the client"] # [derive (Deserialize)] # [serde (tag = "type" , rename_all = "snake_case")] # [allow (clippy :: large_enum_variant)] pub enum ClientMessage { # [doc = " A new connection"] ConnectionInit { # [doc = " Optional init payload from the client"] payload : Option < serde_json :: Value > , } , # [doc = " The start of a Websocket subscription"] # [serde (alias = "subscribe")] Start { # [doc = " Message ID"] id : String , # [doc = " The GraphQL Request - this can be modified by protocol implementors"] # [doc = " to add files uploads."] payload : Request , } , # [doc = " The end of a Websocket subscription"] # [serde (alias = "complete")] Stop { # [doc = " Message ID"] id : String , } , # [doc = " Connection terminated by the client"] ConnectionTerminate , # [doc = " Useful for detecting failed connections, displaying latency metrics or"] # [doc = " other types of network probing."] # [doc = ""] # [doc = " Reference: <https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md#ping>"] Ping { # [doc = " Additional details about the ping."] payload : Option < serde_json :: Value > , } , # [doc = " The response to the Ping message."] # [doc = ""] # [doc = " Reference: <https://github.com/enisdenjo/graphql-ws/blob/master/PROTOCOL.md#pong>"] Pong { # [doc = " Additional details about the pong."] payload : Option < serde_json :: Value > , } , }
};
}
