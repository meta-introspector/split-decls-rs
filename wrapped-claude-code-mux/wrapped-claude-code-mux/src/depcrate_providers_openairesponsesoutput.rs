// Generated macro for ResponsesOutput (struct)
macro_rules! Depcrate_providers_openaiResponsesOutput {
() => {
// Module: crate::providers::openai
// Provides: {"ResponsesOutput"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct ResponsesOutput { # [serde (rename = "type")] output_type : String , # [serde (skip_serializing_if = "Option::is_none")] content : Option < Vec < ResponsesContentBlock > > , }
};
}
