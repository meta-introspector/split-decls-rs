// Generated macro for item_to_string (function)
macro_rules! Depcrateitem_to_string {
() => {
// Module: crate
// Provides: {"item_to_string"}
// Dependencies: {}
pub fn item_to_string (ann : & dyn PpAnn , pat : & hir :: Item < '_ >) -> String { to_string (ann , | s | s . print_item (pat)) }
};
}
