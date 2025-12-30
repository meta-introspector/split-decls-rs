// Generated macro for is_doc_hidden (function)
macro_rules! Depcrate_attrsis_doc_hidden {
() => {
// Module: crate::attrs
// Provides: {"is_doc_hidden"}
// Dependencies: {}
# [doc = " Checks whether `attrs` contain `#[doc(hidden)]`"] pub fn is_doc_hidden (attrs : & [impl AttributeExt]) -> bool { attrs . iter () . filter (| attr | attr . has_name (sym :: doc)) . filter_map (AttributeExt :: meta_item_list) . any (| l | attr :: list_contains_name (& l , sym :: hidden)) }
};
}
