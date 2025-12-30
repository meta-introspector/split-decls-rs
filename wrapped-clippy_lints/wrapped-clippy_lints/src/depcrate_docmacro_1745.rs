// Generated macro for macro_1745 (macro)
macro_rules! Depcrate_docmacro_1745 {
() => {
// Module: crate::doc
// Provides: {"macro_1745"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks the doc comments of publicly visible functions that"] # [doc = " return a `Result` type and warns if there is no `# Errors` section."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Documenting the type of errors that can be returned from a"] # [doc = " function can help callers write code to handle the errors appropriately."] # [doc = ""] # [doc = " ### Examples"] # [doc = " Since the following function returns a `Result` it has an `# Errors` section in"] # [doc = " its doc comment:"] # [doc = ""] # [doc = " ```no_run"] # [doc = "# use std::io;"] # [doc = " /// # Errors"] # [doc = " ///"] # [doc = " /// Will return `Err` if `filename` does not exist or the user does not have"] # [doc = " /// permission to read it."] # [doc = " pub fn read(filename: String) -> io::Result<String> {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.41.0"] pub MISSING_ERRORS_DOC , pedantic , "`pub fn` returns `Result` without `# Errors` in doc comment" }
};
}
