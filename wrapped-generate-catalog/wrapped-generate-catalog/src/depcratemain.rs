// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Error > { let Some (args) = Args :: from_env () ? else { return Ok (()) ; } ; let writer = if let Some (output) = & args . output { Box :: new (File :: create (output) ?) as Box < dyn Write > } else { Box :: new (std :: io :: stdout ()) } ; let mut writer = BufWriter :: new (writer) ; let url = args . url . as_deref () . unwrap_or (CATALOG_URL) ; let html = ureq :: get (url) . call () ? . into_string () ? ; let html = Html :: parse_document (& html) ; let h3_selector = Selector :: parse ("h3") . unwrap () ; let code_selector = Selector :: parse ("code") . unwrap () ; writeln ! (writer , r#"//! CRC algorithms as structs.
use crate::Algorithm;
"#) ? ; for h3 in html . select (& h3_selector) { let h3_a = ElementRef :: wrap (h3 . last_child () . unwrap ()) . unwrap () ; let a_name = h3_a . attr ("name") . unwrap () ; let _name = h3_a . inner_html () ; let mut siblings = h3 . next_siblings () ; let p = next_tag (& mut siblings) ; let Some (parameters) = p . select (& code_selector) . next () . and_then (| node | node . inner_html () . parse () . ok ()) else { break ; } ; let _ul = next_tag (& mut siblings) ; Algorithm { parameters , url : format ! ("{url}#{a_name}") , aliases : vec ! [] , } . emit_rust (& mut writer) ? ; } Ok (()) }
};
}
