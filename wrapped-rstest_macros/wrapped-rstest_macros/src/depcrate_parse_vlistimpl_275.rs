// Generated macro for impl_275 (impl)
macro_rules! Depcrate_parse_vlistimpl_275 {
() => {
// Module: crate::parse::vlist
// Provides: {"impl_275"}
// Dependencies: {}
impl Value { pub (crate) fn new (expr : Expr , description : Option < String >) -> Self { Self { expr , description } } pub (crate) fn description (& self) -> String { self . description . clone () . unwrap_or_else (| | self . expr . to_token_stream () . to_string ()) } }
};
}
