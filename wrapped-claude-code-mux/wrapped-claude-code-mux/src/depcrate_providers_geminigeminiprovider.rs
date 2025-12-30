// Generated macro for GeminiProvider (struct)
macro_rules! Depcrate_providers_geminiGeminiProvider {
() => {
// Module: crate::providers::gemini
// Provides: {"GeminiProvider"}
// Dependencies: {}
# [doc = " Google Gemini provider supporting three authentication methods:"] # [doc = " 1. OAuth 2.0 (Google AI Pro/Ultra) - Uses Code Assist API"] # [doc = " 2. API Key (Google AI Studio) - Uses public Gemini API"] # [doc = " 3. Vertex AI (Google Cloud) - Uses Vertex AI API"] pub struct GeminiProvider { pub name : String , pub api_key : Option < String > , pub base_url : String , pub models : Vec < String > , pub client : Client , pub custom_headers : HashMap < String , String > , pub project_id : Option < String > , pub location : Option < String > , pub oauth_provider_id : Option < String > , pub token_store : Option < TokenStore > , }
};
}
