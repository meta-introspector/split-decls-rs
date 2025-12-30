// Generated macro for local_snippet_without_semicolon (function)
macro_rules! Depcrate_needless_late_initlocal_snippet_without_semicolon {
() => {
// Module: crate::needless_late_init
// Provides: {"local_snippet_without_semicolon"}
// Dependencies: {}
fn local_snippet_without_semicolon (cx : & LateContext < '_ > , local : & LetStmt < '_ >) -> Option < SourceText > { let span = local . span . with_hi (match local . ty { Some (ty) => ty . span . hi () , None => local . pat . span . hi () , }) ; span . get_source_text (cx) }
};
}
