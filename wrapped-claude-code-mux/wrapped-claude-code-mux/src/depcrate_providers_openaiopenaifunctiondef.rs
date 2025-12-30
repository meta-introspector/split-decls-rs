// Generated macro for OpenAIFunctionDef (struct)
macro_rules! Depcrate_providers_openaiOpenAIFunctionDef {
() => {
// Module: crate::providers::openai
// Provides: {"OpenAIFunctionDef"}
// Dependencies: {}
# [doc = " Function definition"] # [derive (Debug , Serialize , Deserialize)] struct OpenAIFunctionDef { name : String , # [serde (skip_serializing_if = "Option::is_none")] description : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] parameters : Option < serde_json :: Value > , }
};
}
