// Generated macro for macro_9653 (macro)
macro_rules! Depcrate_string_patternsmacro_9653 {
() => {
// Module: crate::string_patterns
// Provides: {"macro_9653"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual `char` comparison in string patterns"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more concisely using a `char` or an array of `char`."] # [doc = " This is more readable and more optimized when comparing to only one `char`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \"Hello World!\".trim_end_matches(|c| c == '.' || c == ',' || c == '!' || c == '?');"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " \"Hello World!\".trim_end_matches(['.', ',', '!', '?']);"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub MANUAL_PATTERN_CHAR_COMPARISON , style , "manual char comparison in string patterns" }
};
}
