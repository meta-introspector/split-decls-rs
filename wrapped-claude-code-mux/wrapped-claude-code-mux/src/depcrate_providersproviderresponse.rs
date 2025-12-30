// Generated macro for ProviderResponse (struct)
macro_rules! Depcrate_providersProviderResponse {
() => {
// Module: crate::providers
// Provides: {"ProviderResponse"}
// Dependencies: {}
# [doc = " Provider response that maintains Anthropic API compatibility"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProviderResponse { pub id : String , pub r#type : String , pub role : String , pub content : Vec < ContentBlock > , pub model : String , pub stop_reason : Option < String > , pub stop_sequence : Option < String > , pub usage : Usage , }
};
}
