// Generated macro for extract_retry_delay (function)
macro_rules! Depcrate_providers_geminiextract_retry_delay {
() => {
// Module: crate::providers::gemini
// Provides: {"extract_retry_delay"}
// Dependencies: {}
# [doc = " Extract retry delay from 429 error response"] fn extract_retry_delay (error_text : & str) -> Option < std :: time :: Duration > { if let Ok (error_response) = serde_json :: from_str :: < GeminiErrorResponse > (error_text) { for detail in & error_response . error . details { if let GeminiErrorDetail :: RetryInfo { retry_delay } = detail { if let Some (delay) = parse_retry_delay (retry_delay) { tracing :: info ! ("⏱️  Rate limit hit, will retry after {:?}" , delay) ; return Some (delay) ; } } } for detail in & error_response . error . details { if let GeminiErrorDetail :: ErrorInfo { reason , domain , metadata } = detail { if reason == "RATE_LIMIT_EXCEEDED" && domain . contains ("cloudcode-pa.googleapis.com") { if let Some (quota_reset) = metadata . get ("quotaResetDelay") { if let Some (delay) = parse_retry_delay (quota_reset) { tracing :: info ! ("⏱️  Rate limit hit (RATE_LIMIT_EXCEEDED), will retry after {:?}" , delay) ; return Some (delay) ; } } tracing :: info ! ("⏱️  Rate limit hit (RATE_LIMIT_EXCEEDED), will retry after 10s") ; return Some (std :: time :: Duration :: from_secs (10)) ; } } } } None }
};
}
