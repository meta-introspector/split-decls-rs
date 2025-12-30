// Generated macro for OpenAIContent (enum)
macro_rules! Depcrate_providers_openaiOpenAIContent {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIContent"}
// Dependencies: {}
# [doc = " Content can be string or array of content parts"] # [derive (Debug , Serialize , Deserialize)] # [serde (untagged)] enum OpenAIContent { String (String) , Parts (Vec < OpenAIContentPart >) , }
};
}
