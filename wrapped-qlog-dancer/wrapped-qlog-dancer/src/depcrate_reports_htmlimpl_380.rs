// Generated macro for impl_380 (impl)
macro_rules! Depcrate_reports_htmlimpl_380 {
() => {
// Module: crate::reports::html
// Provides: {"impl_380"}
// Dependencies: {}
impl HtmlVisitorMut for QUICClosureTableDecorator { fn visit_element_mut (& mut self , e : & mut HtmlElement) -> bool { if e . tag () == "tr" { if self . i == 0 { self . i += 1 ; return true ; } let mut mark_red = false ; let mut mark_yellow = false ; let mut mark_green = false ; if let Some (HtmlValue :: Elements (kids)) = e . value () { if let Some (error) = kids . get (2) { if let Some (err) = table_cell_value (error) { if let Ok (val) = err . parse :: < i32 > () { if val == 25 { mark_green = true ; } else if val == 70 { mark_yellow = true ; } else if val == 199 { mark_red = true ; } } } } } if mark_yellow { let mut attrs = e . attrs () . to_vec () ; attrs . push (Attribute :: new ("class" , "yellow" . to_string ())) ; * e = HtmlElement :: new ("tr" , attrs , e . value () . cloned ()) ; } else if mark_red { let mut attrs = e . attrs () . to_vec () ; attrs . push (Attribute :: new ("class" , "red" . to_string ())) ; * e = HtmlElement :: new ("tr" , attrs , e . value () . cloned ()) ; } else if mark_green { let mut attrs = e . attrs () . to_vec () ; attrs . push (Attribute :: new ("class" , "green" . to_string ())) ; * e = HtmlElement :: new ("tr" , attrs , e . value () . cloned ()) ; } } true } }
};
}
