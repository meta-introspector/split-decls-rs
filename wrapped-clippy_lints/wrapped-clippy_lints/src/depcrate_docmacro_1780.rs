// Generated macro for macro_1780 (macro)
macro_rules! Depcrate_docmacro_1780 {
() => {
// Module: crate::doc
// Provides: {"macro_1780"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for doc comments whose paragraphs do not end with a period or another punctuation mark."] # [doc = " Various Markdowns constructs are taken into account to avoid false positives."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " A project may wish to enforce consistent doc comments by making sure paragraphs end with a"] # [doc = " punctuation mark."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " /// Returns a random number"] # [doc = " ///"] # [doc = " /// It was chosen by a fair dice roll"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " /// Returns a random number."] # [doc = " ///"] # [doc = " /// It was chosen by a fair dice roll."] # [doc = " ```"] # [clippy :: version = "1.93.0"] pub DOC_PARAGRAPHS_MISSING_PUNCTUATION , restriction , "missing terminal punctuation in doc comments" }
};
}
