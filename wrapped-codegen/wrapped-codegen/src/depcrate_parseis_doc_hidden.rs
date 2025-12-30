// Generated macro for is_doc_hidden (function)
macro_rules! Depcrate_parseis_doc_hidden {
() => {
// Module: crate::parse
// Provides: {"is_doc_hidden"}
// Dependencies: {}
fn is_doc_hidden (attrs : & [Attribute]) -> bool { for attr in attrs { if attr . path () . is_ident ("doc") && attr . parse_args :: < parsing :: kw :: hidden > () . is_ok () { return true ; } } false }
};
}
