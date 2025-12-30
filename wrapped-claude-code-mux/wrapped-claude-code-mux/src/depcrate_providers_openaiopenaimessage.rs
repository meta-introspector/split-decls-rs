// Generated macro for OpenAIMessage (struct)
macro_rules! Depcrate_providers_openaiOpenAIMessage {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIMessage"}
// Dependencies: {}
# [derive (Debug , Serialize , Deserialize)] struct OpenAIMessage { role : String , # [serde (skip_serializing_if = "Option::is_none")] content : Option < OpenAIContent > , # [serde (skip_serializing_if = "Option::is_none")] reasoning : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] tool_calls : Option < Vec < OpenAIToolCall > > , # [serde (skip_serializing_if = "Option::is_none")] tool_call_id : Option < String > , }
};
}
