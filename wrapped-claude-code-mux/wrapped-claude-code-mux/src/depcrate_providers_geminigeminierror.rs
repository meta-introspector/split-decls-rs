// Generated macro for GeminiError (struct)
macro_rules! Depcrate_providers_geminiGeminiError {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiError"}
// Dependencies: {}
# [derive (Debug , Deserialize)] struct GeminiError { code : u16 , message : String , status : String , # [serde (default)] details : Vec < GeminiErrorDetail > , }
};
}
