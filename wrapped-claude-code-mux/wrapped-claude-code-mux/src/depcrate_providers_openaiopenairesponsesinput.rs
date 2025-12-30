// Generated macro for OpenAIResponsesInput (enum)
macro_rules! Depcrate_providers_openaiOpenAIResponsesInput {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIResponsesInput"}
// Dependencies: {}
# [doc = " Input for Responses API can be string or array of messages"] # [derive (Debug , Serialize)] # [serde (untagged)] enum OpenAIResponsesInput { Text (String) , Messages (Vec < OpenAIResponsesMessage >) , }
};
}
