// Generated macro for AnthropicResponse (struct)
macro_rules! Depcrate_server_openai_compatAnthropicResponse {
() => {
// Module: crate::server::openai_compat
// Provides: {"AnthropicResponse"}
// Dependencies: {}
# [derive (Debug , Clone , Deserialize , Serialize)] pub struct AnthropicResponse { pub id : String , pub r#type : String , pub role : String , pub model : String , # [serde (skip_serializing_if = "Option::is_none")] pub stop_reason : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub stop_sequence : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub usage : Option < Usage > , pub content : Vec < MessageContent > , }
};
}
