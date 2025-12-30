// Generated macro for ClientMessage (enum)
macro_rules! Depcrate_graphql_ws_client_messageClientMessage {
() => {
// Module: crate::graphql_ws::client_message
// Provides: {"ClientMessage"}
// Dependencies: {}
# [doc = " ClientMessage defines the message types that clients can send."] # [derive (Debug , Deserialize , PartialEq)] # [serde (bound (deserialize = "S: Deserialize<'de>"))] # [serde (rename_all = "snake_case")] # [serde (tag = "type")] pub enum ClientMessage < S > { # [doc = " ConnectionInit is sent by the client upon connecting."] ConnectionInit { # [doc = " Optional parameters of any type sent from the client. These are often used for"] # [doc = " authentication."] # [serde (default , deserialize_with = "default_for_null")] payload : Variables < S > , } , # [doc = " Start messages are used to execute a GraphQL operation."] Start { # [doc = " The id of the operation. This can be anything, but must be unique. If there are other"] # [doc = " in-flight operations with the same id, the message will be ignored or cause an error."] id : String , # [doc = " The query, variables, and operation name."] payload : StartPayload < S > , } , # [doc = " Stop messages are used to unsubscribe from a subscription."] Stop { # [doc = " The id of the operation to stop."] id : String , } , # [doc = " ConnectionTerminate is used to terminate the connection."] ConnectionTerminate , }
};
}
