// Generated macro for OpenAIRequest (struct)
macro_rules! Depcrate_server_openai_compatOpenAIRequest {
() => {
// Module: crate::server::openai_compat
// Provides: {"OpenAIRequest"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct OpenAIRequest { pub model : String , pub messages : Vec < OpenAIMessage > , # [serde (default)] pub stream : bool , }
};
}
