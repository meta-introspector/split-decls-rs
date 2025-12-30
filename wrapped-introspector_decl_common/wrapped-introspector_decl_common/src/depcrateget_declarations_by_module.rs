// Generated macro for get_declarations_by_module (function)
macro_rules! Depcrateget_declarations_by_module {
() => {
// Module: crate
// Provides: {"get_declarations_by_module"}
// Dependencies: {}
pub fn get_declarations_by_module (module : & str) -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | { r . by_module . get (module) . map (| indices | { indices . iter () . filter_map (| & i | r . declarations . get (i) . cloned ()) . collect () }) . unwrap_or_default () }) . unwrap_or_default () }
};
}
