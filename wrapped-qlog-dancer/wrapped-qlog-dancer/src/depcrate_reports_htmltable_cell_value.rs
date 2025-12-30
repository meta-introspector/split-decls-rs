// Generated macro for table_cell_value (function)
macro_rules! Depcrate_reports_htmltable_cell_value {
() => {
// Module: crate::reports::html
// Provides: {"table_cell_value"}
// Dependencies: {}
fn table_cell_value (cell : & HtmlElement) -> Option < String > { if cell . tag () == "td" { if let Some (HtmlValue :: Elements (elems)) = cell . value () { if let Some (val) = elems . first () { if let Some (HtmlValue :: Elements (p)) = val . value () { if let Some (p_val) = p . first () { if let Some (HtmlValue :: Content (inner)) = p_val . value () { return Some (inner . clone ()) ; } } } } } } None }
};
}
