// Generated macro for ConnectionErrorPayload (struct)
macro_rules! Depcrate_graphql_ws_server_messageConnectionErrorPayload {
() => {
// Module: crate::graphql_ws::server_message
// Provides: {"ConnectionErrorPayload"}
// Dependencies: {}
# [doc = " The payload for errors that are not associated with a GraphQL operation."] # [derive (Debug , Eq , PartialEq , Serialize)] # [serde (rename_all = "camelCase")] pub struct ConnectionErrorPayload { # [doc = " The error message."] pub message : String , }
};
}
