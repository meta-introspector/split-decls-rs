// Generated macro for has_any_fn (function)
macro_rules! Depcrate_utilshas_any_fn {
() => {
// Module: crate::utils
// Provides: {"has_any_fn"}
// Dependencies: {}
fn has_any_fn (imp : & ast :: Impl , names : & [String]) -> bool { if let Some (il) = imp . assoc_item_list () { for item in il . assoc_items () { if let ast :: AssocItem :: Fn (f) = item && let Some (name) = f . name () && names . iter () . any (| n | n . eq_ignore_ascii_case (& name . text ())) { return true ; } } } false }
};
}
