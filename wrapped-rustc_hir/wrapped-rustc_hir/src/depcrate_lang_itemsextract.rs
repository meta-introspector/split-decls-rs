// Generated macro for extract (function)
macro_rules! Depcrate_lang_itemsextract {
() => {
// Module: crate::lang_items
// Provides: {"extract"}
// Dependencies: {}
# [doc = " Extracts the first `lang = \"$name\"` out of a list of attributes."] # [doc = " The `#[panic_handler]` attribute is also extracted out when found."] pub fn extract (attrs : & [impl AttributeExt]) -> Option < (Symbol , Span) > { attrs . iter () . find_map (| attr | { Some (match attr { _ if attr . has_name (sym :: lang) => (attr . value_str () ? , attr . span ()) , _ if attr . has_name (sym :: panic_handler) => (sym :: panic_impl , attr . span ()) , _ => return None , }) }) }
};
}
