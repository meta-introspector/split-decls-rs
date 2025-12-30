// Generated macro for macro_9694 (macro)
macro_rules! Depcrate_semicolon_if_nothing_returnedmacro_9694 {
() => {
// Module: crate::semicolon_if_nothing_returned
// Provides: {"macro_9694"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for blocks of expressions and fires if the last expression returns"] # [doc = " `()` but is not followed by a semicolon."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The semicolon might be optional but when extending the block with new"] # [doc = " code, it doesn't require a change in previous last line."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     println!(\"Hello world\")"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     println!(\"Hello world\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub SEMICOLON_IF_NOTHING_RETURNED , pedantic , "add a semicolon if nothing is returned" }
};
}
