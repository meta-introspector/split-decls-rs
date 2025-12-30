// Generated macro for ServerMessage (enum)
macro_rules! Depcrate_graphql_transport_ws_server_messageServerMessage {
() => {
// Module: crate::graphql_transport_ws::server_message
// Provides: {"ServerMessage"}
// Dependencies: {}
# [doc = " ServerMessage defines the message types that servers can send."] # [derive (Debug , PartialEq , Serialize)] # [serde (rename_all = "snake_case")] # [serde (tag = "type")] pub enum ServerMessage < S > { # [doc = " ConnectionAck is sent in response to a client's ConnectionInit message if the server accepted a"] # [doc = " connection."] ConnectionAck , # [doc = " The response to the `Ping` message."] Pong , # [doc = " Data contains the result of a query, mutation, or subscription event."] Next { # [doc = " The id of the operation that the data is for."] id : String , # [doc = " The data and errors that occurred during execution."] payload : NextPayload < S > , } , # [doc = " Error contains an error that occurs before execution, such as validation errors."] Error { # [doc = " The id of the operation that triggered this error."] id : String , # [doc = " The error(s)."] payload : ErrorPayload , } , # [doc = " Complete indicates that no more data will be sent for the given operation."] Complete { # [doc = " The id of the operation that has completed."] id : String , } , }
};
}
