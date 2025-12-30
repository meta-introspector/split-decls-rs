// Generated macro for inject_table_id_class (function)
macro_rules! Depcrate_reports_htmlinject_table_id_class {
() => {
// Module: crate::reports::html
// Provides: {"inject_table_id_class"}
// Dependencies: {}
fn inject_table_id_class (input : & HtmlTable , id : Option < String > , class : Option < String > ,) -> String { let id = if let Some (i) = id { format ! ("id='{i}'") } else { "" . to_string () } ; let class = if let Some (c) = class { format ! ("class='{c}'") } else { "" . to_string () } ; let replaced = format ! ("<table {id} {class}>") ; input . to_string () . replace ("<table>" , & replaced) }
};
}
