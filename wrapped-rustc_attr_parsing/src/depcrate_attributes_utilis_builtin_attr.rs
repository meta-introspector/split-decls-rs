// Generated macro for is_builtin_attr (function)
macro_rules! Depcrate_attributes_utilis_builtin_attr {
() => {
// Module: crate::attributes::util
// Provides: {"is_builtin_attr"}
// Dependencies: {}
pub fn is_builtin_attr (attr : & impl AttributeExt) -> bool { attr . is_doc_comment () || attr . ident () . is_some_and (| ident | is_builtin_attr_name (ident . name)) }
};
}
