// Generated macro for GeminiTool (enum)
macro_rules! Depcrate_providers_geminiGeminiTool {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiTool"}
// Dependencies: {}
# [doc = " Gemini Tool supports multiple tool types via protobuf oneof"] # [derive (Debug , Clone , Serialize)] # [serde (untagged)] enum GeminiTool { # [doc = " Function calling tools"] FunctionDeclarations { # [serde (rename = "functionDeclarations")] function_declarations : Vec < GeminiFunctionDeclaration > , } , # [doc = " Google Search tool"] GoogleSearch { # [serde (rename = "googleSearch")] google_search : GoogleSearchTool , } , # [doc = " URL Context/Fetch tool"] UrlContext { # [serde (rename = "urlContext")] url_context : UrlContextTool , } , }
};
}
