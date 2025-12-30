// Generated macro for under_name (function)
macro_rules! Depcrate_genunder_name {
() => {
// Module: crate::gen
// Provides: {"under_name"}
// Dependencies: {}
pub fn under_name (name : & str) -> Ident { Ident :: new (& name . to_snake_case () , Span :: call_site ()) }
};
}
