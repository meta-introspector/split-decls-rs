// Generated macro for output (function)
macro_rules! Depcrate_jsonoutput {
() => {
// Module: crate::json
// Provides: {"output"}
// Dependencies: {}
# [doc = " Creates the log file output for [`crate::config::OutputFormat::Json`]"] pub (crate) fn output (clippy_warnings : Vec < ClippyWarning >) -> String { let mut lints : Vec < LintJson > = clippy_warnings . into_iter () . map (| warning | { let span = warning . span () ; let file_name = span . file_name . strip_prefix ("target/lintcheck/sources/") . unwrap_or (& span . file_name) ; let file_line = format ! ("{file_name}:{}" , span . line_start) ; LintJson { name : warning . name , file_line , file_url : warning . url , rendered : warning . diag . rendered . unwrap () . trim () . to_string () , } }) . collect () ; lints . sort_by (| a , b | a . key () . cmp (& b . key ())) ; serde_json :: to_string (& lints) . unwrap () }
};
}
