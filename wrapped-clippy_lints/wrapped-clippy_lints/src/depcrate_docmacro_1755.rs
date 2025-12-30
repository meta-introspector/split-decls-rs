// Generated macro for macro_1755 (macro)
macro_rules! Depcrate_docmacro_1755 {
() => {
// Module: crate::doc
// Provides: {"macro_1755"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Detects overindented list items in doc comments where the continuation"] # [doc = " lines are indented more than necessary."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Overindented list items in doc comments can lead to inconsistent and"] # [doc = " poorly formatted documentation when rendered. Excessive indentation may"] # [doc = " cause the text to be misinterpreted as a nested list item or code block,"] # [doc = " affecting readability and the overall structure of the documentation."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " /// - This is the first item in a list"] # [doc = " ///      and this line is overindented."] # [doc = " # fn foo() {}"] # [doc = " ```"] # [doc = ""] # [doc = " Fixes this into:"] # [doc = " ```no_run"] # [doc = " /// - This is the first item in a list"] # [doc = " ///   and this line is overindented."] # [doc = " # fn foo() {}"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub DOC_OVERINDENTED_LIST_ITEMS , style , "ensure list items are not overindented" }
};
}
