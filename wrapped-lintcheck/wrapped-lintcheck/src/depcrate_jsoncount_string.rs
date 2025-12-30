// Generated macro for count_string (function)
macro_rules! Depcrate_jsoncount_string {
() => {
// Module: crate::json
// Provides: {"count_string"}
// Dependencies: {}
# [doc = " This generates the `x added` string for the start of the job summery."] # [doc = " It linkifies them if possible to jump to the respective heading."] fn count_string (lint : & str , label : & str , count : usize) -> String { if count == 0 { format ! ("0 {label}") } else { let html_id = to_html_id (lint) ; format ! ("[{count} {label}](#{html_id}-{label})") } }
};
}
