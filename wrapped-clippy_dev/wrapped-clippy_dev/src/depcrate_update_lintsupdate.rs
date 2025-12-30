// Generated macro for update (function)
macro_rules! Depcrate_update_lintsupdate {
() => {
// Module: crate::update_lints
// Provides: {"update"}
// Dependencies: {}
# [doc = " Runs the `update_lints` command."] # [doc = ""] # [doc = " This updates various generated values from the lint source code."] # [doc = ""] # [doc = " `update_mode` indicates if the files should be updated or if updates should be checked for."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if a file path could not read from or then written to"] pub fn update (cx : ParseCx < '_ > , update_mode : UpdateMode) { let lints = cx . find_lint_decls () ; let (deprecated , renamed) = cx . read_deprecated_lints () ; generate_lint_files (update_mode , & lints , & deprecated , & renamed) ; }
};
}
