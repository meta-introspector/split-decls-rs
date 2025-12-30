// Generated macro for MissingDoc (struct)
macro_rules! Depcrate_missing_docMissingDoc {
() => {
// Module: crate::missing_doc
// Provides: {"MissingDoc"}
// Dependencies: {}
pub struct MissingDoc { # [doc = " Whether to **only** check for missing documentation in items visible within the current"] # [doc = " crate. For example, `pub(crate)` items."] crate_items_only : bool , # [doc = " Whether to allow fields starting with an underscore to skip documentation requirements"] allow_unused : bool , # [doc = " The current number of modules since the crate root."] module_depth : u32 , macro_module_depth : u32 , # [doc = " The current level of the attribute stack."] attr_depth : u32 , # [doc = " What `attr_depth` level the first `doc(hidden)` attribute was seen. This is zero if the"] # [doc = " attribute hasn't been seen."] doc_hidden_depth : u32 , # [doc = " What `attr_depth` level the first `automatically_derived` attribute was seen. This is zero"] # [doc = " if the attribute hasn't been seen."] automatically_derived_depth : u32 , # [doc = " The id of the first body we've seen."] in_body : Option < BodyId > , # [doc = " The module/crate id an item must be visible at to be linted."] require_visibility_at : Option < LocalDefId > , }
};
}
