// Generated macro for print_lint_warnings (function)
macro_rules! Depcrate_jsonprint_lint_warnings {
() => {
// Module: crate::json
// Provides: {"print_lint_warnings"}
// Dependencies: {}
fn print_lint_warnings (lint : & LintWarnings , truncate_after : usize) { let name = & lint . name ; let html_id = to_html_id (name) ; println ! (r#"<h2 id="{html_id}"><code>{name}</code></h2>"#) ; println ! () ; print ! (r"{}, {}, {}" , count_string (name , "added" , lint . added . len ()) , count_string (name , "removed" , lint . removed . len ()) , count_string (name , "changed" , lint . changed . len ()) ,) ; println ! () ; print_warnings ("Added" , & lint . added , truncate_after / 3) ; print_warnings ("Removed" , & lint . removed , truncate_after / 3) ; print_changed_diff (& lint . changed , truncate_after / 3) ; }
};
}
