// Generated macro for StartPayload (struct)
macro_rules! Depcrate_graphql_ws_client_messageStartPayload {
() => {
// Module: crate::graphql_ws::client_message
// Provides: {"StartPayload"}
// Dependencies: {}
# [doc = " The payload for a client's \"start\" message. This triggers execution of a query, mutation, or"] # [doc = " subscription."] # [derive (Debug , Deserialize , PartialEq)] # [serde (bound (deserialize = "S: Deserialize<'de>"))] # [serde (rename_all = "camelCase")] pub struct StartPayload < S > { # [doc = " The document body."] pub query : String , # [doc = " The optional variables."] # [serde (default , deserialize_with = "default_for_null")] pub variables : Variables < S > , # [doc = " The optional operation name (required if the document contains multiple operations)."] pub operation_name : Option < String > , }
};
}
