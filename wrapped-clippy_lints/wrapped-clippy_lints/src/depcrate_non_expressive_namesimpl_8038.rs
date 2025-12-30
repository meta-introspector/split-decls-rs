// Generated macro for impl_8038 (impl)
macro_rules! Depcrate_non_expressive_namesimpl_8038 {
() => {
// Module: crate::non_expressive_names
// Provides: {"impl_8038"}
// Dependencies: {}
impl SimilarNamesLocalVisitor < '_ , '_ > { fn check_single_char_names (& self) { if self . single_char_names . last () . map (Vec :: len) == Some (0) { return ; } let num_single_char_names = self . single_char_names . iter () . flatten () . count () ; if num_single_char_names as u64 > self . threshold { let span = self . single_char_names . iter () . flatten () . map (| ident | ident . span) . collect :: < Vec < _ > > () ; span_lint (self . cx , MANY_SINGLE_CHAR_NAMES , span , format ! ("{num_single_char_names} bindings with single-character names in scope") ,) ; } } }
};
}
