// Generated macro for SubscribePayload (struct)
macro_rules! Depcrate_graphql_transport_ws_client_messageSubscribePayload {
() => {
// Module: crate::graphql_transport_ws::client_message
// Provides: {"SubscribePayload"}
// Dependencies: {}
# [doc = " The payload for a client's \"start\" message. This triggers execution of a query, mutation, or"] # [doc = " subscription."] # [derive (Debug , Deserialize , PartialEq)] # [serde (bound (deserialize = "S: Deserialize<'de>"))] # [serde (rename_all = "camelCase")] pub struct SubscribePayload < S > { # [doc = " The document body."] pub query : String , # [doc = " The optional variables."] # [serde (default , deserialize_with = "default_for_null")] pub variables : Variables < S > , # [doc = " The optional operation name (required if the document contains multiple operations)."] pub operation_name : Option < String > , # [doc = " The optional extension data."] # [serde (default , deserialize_with = "default_for_null")] pub extensions : Variables < S > , }
};
}
