// Generated macro for OpenAIResponsesMessage (struct)
macro_rules! Depcrate_providers_openaiOpenAIResponsesMessage {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIResponsesMessage"}
// Dependencies: {}
# [doc = " Message format for Responses API"] # [derive (Debug , Serialize)] struct OpenAIResponsesMessage { role : String , # [serde (skip_serializing_if = "Option::is_none")] content : Option < String > , }
};
}
