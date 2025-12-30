// Generated macro for get_export (function)
macro_rules! Depcrateget_export {
() => {
// Module: crate
// Provides: {"get_export"}
// Dependencies: {}
fn get_export (attributes : & [Attribute]) -> Option < & Attribute > { attributes . iter () . find (| & attr | attr . path () . is_ident (& format_ident ! ("export"))) }
};
}
