// Generated macro for impl_3253 (impl)
macro_rules! Depcrate_large_include_fileimpl_3253 {
() => {
// Module: crate::large_include_file
// Provides: {"impl_3253"}
// Dependencies: {}
impl EarlyLintPass for LargeIncludeFile { fn check_attribute (& mut self , cx : & EarlyContext < '_ > , attr : & Attribute) { if ! attr . span . from_expansion () && let AttrKind :: Normal (ref item) = attr . kind && let Some (doc) = attr . doc_str () && doc . as_str () . len () as u64 > self . max_file_size && let AttrArgs :: Eq { expr : meta , .. } = & item . item . args && ! attr . span . contains (meta . span) && let Some (snippet) = snippet_opt (cx , attr . span) && let Some (start) = snippet . find ('[') && let Some (end) = snippet . rfind (']') && let snippet = & snippet [start + 1 .. end] && let Some (sub_snippet) = snippet . trim () . strip_prefix ("doc") && let Some (sub_snippet) = sub_snippet . trim () . strip_prefix ("=") && let sub_snippet = sub_snippet . trim () && (sub_snippet . starts_with ("include_str!") || sub_snippet . starts_with ("include_bytes!")) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , LARGE_INCLUDE_FILE , attr . span , "attempted to include a large file" , | diag | { diag . note (format ! ("the configuration allows a maximum size of {} bytes" , self . max_file_size)) ; } ,) ; } } }
};
}
