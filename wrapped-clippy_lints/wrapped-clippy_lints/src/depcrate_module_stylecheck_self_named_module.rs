// Generated macro for check_self_named_module (function)
macro_rules! Depcrate_module_stylecheck_self_named_module {
() => {
// Module: crate::module_style
// Provides: {"check_self_named_module"}
// Dependencies: {}
fn check_self_named_module (cx : & EarlyContext < '_ > , path : & Path , file : & SourceFile) { if ! path . ends_with ("mod.rs") { let mut mod_folder = path . with_extension ("") ; span_lint_and_then (cx , SELF_NAMED_MODULE_FILES , Span :: new (file . start_pos , file . start_pos , SyntaxContext :: root () , None) , format ! ("`mod.rs` files are required, found `{}`" , path . display ()) , | diag | { mod_folder . push ("mod.rs") ; diag . help (format ! ("move `{}` to `{}`" , path . display () , mod_folder . display ())) ; } ,) ; } }
};
}
