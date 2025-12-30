// Generated macro for parse_retry_delay (function)
macro_rules! Depcrate_providers_geminiparse_retry_delay {
() => {
// Module: crate::providers::gemini
// Provides: {"parse_retry_delay"}
// Dependencies: {}
# [doc = " Parse retry delay from Google's duration format (e.g., \"3.020317815s\", \"60s\", \"900ms\")"] fn parse_retry_delay (duration : & str) -> Option < std :: time :: Duration > { if let Some (ms_str) = duration . strip_suffix ("ms") { ms_str . parse :: < f64 > () . ok () . map (| ms | std :: time :: Duration :: from_millis (ms as u64)) } else if let Some (s_str) = duration . strip_suffix ("s") { s_str . parse :: < f64 > () . ok () . map (| s | std :: time :: Duration :: from_secs_f64 (s)) } else { None } }
};
}
