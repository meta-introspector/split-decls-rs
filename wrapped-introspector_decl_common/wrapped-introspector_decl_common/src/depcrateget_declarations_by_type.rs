// Generated macro for get_declarations_by_type (function)
macro_rules! Depcrateget_declarations_by_type {
() => {
// Module: crate
// Provides: {"get_declarations_by_type"}
// Dependencies: {}
pub fn get_declarations_by_type (node_type : & str) -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | { r . by_type . get (node_type) . map (| indices | { indices . iter () . filter_map (| & i | r . declarations . get (i) . cloned ()) . collect () }) . unwrap_or_default () }) . unwrap_or_default () }
};
}
