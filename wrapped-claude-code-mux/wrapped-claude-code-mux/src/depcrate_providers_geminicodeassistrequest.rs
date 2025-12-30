// Generated macro for CodeAssistRequest (struct)
macro_rules! Depcrate_providers_geminiCodeAssistRequest {
() => {
// Module: crate::providers::gemini
// Provides: {"CodeAssistRequest"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize)] struct CodeAssistRequest { model : String , # [serde (skip_serializing_if = "Option::is_none")] project : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] user_prompt_id : Option < String > , request : CodeAssistInnerRequest , }
};
}
