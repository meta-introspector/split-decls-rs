// Generated macro for doc_hint_of_variant (function)
macro_rules! Depcrate_item_enumdoc_hint_of_variant {
() => {
// Module: crate::item_enum
// Provides: {"doc_hint_of_variant"}
// Dependencies: {}
fn doc_hint_of_variant (variant : & syn :: Variant) -> String { let mut text = find_doc_hint (& variant . attrs) . unwrap_or (variant . ident . to_string ()) ; if unstable_of_variant (& variant) { text . push_str (" (unstable)") } ; text }
};
}
