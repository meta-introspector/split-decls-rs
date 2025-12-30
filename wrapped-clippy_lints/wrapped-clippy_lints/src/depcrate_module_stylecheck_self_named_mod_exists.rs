// Generated macro for check_self_named_mod_exists (function)
macro_rules! Depcrate_module_stylecheck_self_named_mod_exists {
() => {
// Module: crate::module_style
// Provides: {"check_self_named_mod_exists"}
// Dependencies: {}
# [doc = " Checks every path for the presence of `mod.rs` files and emits the lint if found."] # [doc = " We should not emit a lint for test modules in the presence of `mod.rs`."] # [doc = " Using `mod.rs` in integration tests is a [common pattern](https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-test)"] # [doc = " for code-sharing between tests."] fn check_self_named_mod_exists (cx : & EarlyContext < '_ > , path : & Path , file : & SourceFile) { if path . ends_with ("mod.rs") && ! path . starts_with ("tests") { span_lint_and_then (cx , MOD_MODULE_FILES , Span :: new (file . start_pos , file . start_pos , SyntaxContext :: root () , None) , format ! ("`mod.rs` files are not allowed, found `{}`" , path . display ()) , | diag | { let mut mod_file = path . to_path_buf () ; mod_file . pop () ; mod_file . set_extension ("rs") ; diag . help (format ! ("move `{}` to `{}`" , path . display () , mod_file . display ())) ; } ,) ; } }
};
}
