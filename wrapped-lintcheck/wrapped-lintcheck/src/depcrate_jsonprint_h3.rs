// Generated macro for print_h3 (function)
macro_rules! Depcrate_jsonprint_h3 {
() => {
// Module: crate::json
// Provides: {"print_h3"}
// Dependencies: {}
fn print_h3 (lint : & str , title : & str) { let html_id = to_html_id (lint) ; println ! (r#"<h3 id="{html_id}-{title}">{title}</h3>"#) ; }
};
}
