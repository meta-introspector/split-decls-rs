// Generated macro for rustfix_diagnostics_only (function)
macro_rules! Depcrate_executor_jsonrustfix_diagnostics_only {
() => {
// Module: crate::executor::json
// Provides: {"rustfix_diagnostics_only"}
// Dependencies: {}
pub fn rustfix_diagnostics_only (output : & str) -> String { output . lines () . filter (| line | line . starts_with ('{') && serde_json :: from_str :: < Diagnostic > (line) . is_ok ()) . collect () }
};
}
