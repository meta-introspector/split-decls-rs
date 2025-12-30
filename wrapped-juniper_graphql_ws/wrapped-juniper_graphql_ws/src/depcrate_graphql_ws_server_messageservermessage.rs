// Generated macro for ServerMessage (enum)
macro_rules! Depcrate_graphql_ws_server_messageServerMessage {
() => {
// Module: crate::graphql_ws::server_message
// Provides: {"ServerMessage"}
// Dependencies: {}
# [doc = " ServerMessage defines the message types that servers can send."] # [derive (Debug , PartialEq , Serialize)] # [serde (rename_all = "snake_case")] # [serde (tag = "type")] pub enum ServerMessage < S > { # [doc = " ConnectionError is used for errors that are not associated with a GraphQL operation. For"] # [doc = " example, this will be used when:"] # [doc = ""] # [doc = "   * The server is unable to parse a client's message."] # [doc = "   * The client's initialization parameters are rejected."] ConnectionError { # [doc = " The error that occurred."] payload : ConnectionErrorPayload , } , # [doc = " ConnectionAck is sent in response to a client's ConnectionInit message if the server accepted a"] # [doc = " connection."] ConnectionAck , # [doc = " Data contains the result of a query, mutation, or subscription event."] Data { # [doc = " The id of the operation that the data is for."] id : String , # [doc = " The data and errors that occurred during execution."] payload : DataPayload < S > , } , # [doc = " Error contains an error that occurs before execution, such as validation errors."] Error { # [doc = " The id of the operation that triggered this error."] id : String , # [doc = " The error(s)."] payload : ErrorPayload , } , # [doc = " Complete indicates that no more data will be sent for the given operation."] Complete { # [doc = " The id of the operation that has completed."] id : String , } , # [doc = " ConnectionKeepAlive is sent periodically after accepting a connection."] # [serde (rename = "ka")] ConnectionKeepAlive , }
};
}
