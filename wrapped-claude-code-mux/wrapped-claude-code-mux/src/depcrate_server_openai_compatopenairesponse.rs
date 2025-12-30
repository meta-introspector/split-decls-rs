// Generated macro for OpenAIResponse (struct)
macro_rules! Depcrate_server_openai_compatOpenAIResponse {
() => {
// Module: crate::server::openai_compat
// Provides: {"OpenAIResponse"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize)] pub struct OpenAIResponse { pub id : String , pub object : String , pub created : u64 , pub model : String , pub choices : Vec < OpenAIChoice > , pub usage : OpenAIUsage , }
};
}
