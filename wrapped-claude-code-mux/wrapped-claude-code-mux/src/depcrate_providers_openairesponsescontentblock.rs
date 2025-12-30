// Generated macro for ResponsesContentBlock (struct)
macro_rules! Depcrate_providers_openaiResponsesContentBlock {
() => {
// Module: crate::providers::openai
// Provides: {"ResponsesContentBlock"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct ResponsesContentBlock { # [serde (rename = "type")] block_type : String , # [serde (skip_serializing_if = "Option::is_none")] text : Option < String > , }
};
}
