// Generated macro for NextPayload (struct)
macro_rules! Depcrate_graphql_transport_ws_server_messageNextPayload {
() => {
// Module: crate::graphql_transport_ws::server_message
// Provides: {"NextPayload"}
// Dependencies: {}
# [doc = " Sent after execution of an operation. For queries and mutations, this is sent to the client"] # [doc = " once. For subscriptions, this is sent for every event in the event stream."] # [derive (Debug , PartialEq , Serialize)] # [serde (rename_all = "camelCase")] pub struct NextPayload < S > { # [doc = " The result data."] pub data : Value < S > , # [doc = " The errors that have occurred during execution. Note that parse and validation errors are"] # [doc = " not included here. They are sent via Error messages."] # [serde (skip_serializing_if = "Vec::is_empty")] pub errors : Vec < ExecutionError < S > > , }
};
}
