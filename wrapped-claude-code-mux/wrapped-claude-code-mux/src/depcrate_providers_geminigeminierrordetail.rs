// Generated macro for GeminiErrorDetail (enum)
macro_rules! Depcrate_providers_geminiGeminiErrorDetail {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiErrorDetail"}
// Dependencies: {}
# [derive (Debug , Deserialize)] # [serde (tag = "@type")] enum GeminiErrorDetail { # [serde (rename = "type.googleapis.com/google.rpc.RetryInfo")] RetryInfo { # [serde (rename = "retryDelay")] retry_delay : String } , # [serde (rename = "type.googleapis.com/google.rpc.ErrorInfo")] ErrorInfo { reason : String , domain : String , # [serde (default)] metadata : HashMap < String , String > , } , # [serde (other)] Unknown , }
};
}
