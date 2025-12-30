// Generated macro for macro_1882 (macro)
macro_rules! Depcrate_empty_line_aftermacro_1882 {
() => {
// Module: crate::empty_line_after
// Provides: {"macro_1882"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty lines after doc comments."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The doc comment may have meant to be an inner doc comment, regular"] # [doc = " comment or applied to some old code that is now commented out. If it was"] # [doc = " intended to be a doc comment, then the empty line should be removed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " /// Some doc comment with a blank line after it."] # [doc = ""] # [doc = " fn f() {}"] # [doc = ""] # [doc = " /// Docs for `old_code`"] # [doc = " // fn old_code() {}"] # [doc = ""] # [doc = " fn new_code() {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " //! Convert it to an inner doc comment"] # [doc = ""] # [doc = " // Or a regular comment"] # [doc = ""] # [doc = " /// Or remove the empty line"] # [doc = " fn f() {}"] # [doc = ""] # [doc = " // /// Docs for `old_code`"] # [doc = " // fn old_code() {}"] # [doc = ""] # [doc = " fn new_code() {}"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub EMPTY_LINE_AFTER_DOC_COMMENTS , suspicious , "empty line after doc comments" }
};
}
