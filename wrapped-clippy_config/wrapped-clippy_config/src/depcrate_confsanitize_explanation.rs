// Generated macro for sanitize_explanation (function)
macro_rules! Depcrate_confsanitize_explanation {
() => {
// Module: crate::conf
// Provides: {"sanitize_explanation"}
// Dependencies: {}
pub fn sanitize_explanation (raw_docs : & str) -> String { let mut explanation = String :: with_capacity (128) ; let mut in_code = false ; for line in raw_docs . lines () { let line = line . strip_prefix (' ') . unwrap_or (line) ; if let Some (lang) = line . strip_prefix ("```") { let tag = lang . split_once (',') . map_or (lang , | (left , _) | left) ; if ! in_code && matches ! (tag , "" | "rust" | "ignore" | "should_panic" | "no_run" | "compile_fail") { explanation += "```rust\n" ; } else { explanation += line ; explanation . push ('\n') ; } in_code = ! in_code ; } else if ! (in_code && line . starts_with ("# ")) { explanation += line ; explanation . push ('\n') ; } } explanation }
};
}
