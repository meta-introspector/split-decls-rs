// Generated macro for macro_7229 (macro)
macro_rules! Depcrate_methodsmacro_7229 {
() => {
// Module: crate::methods
// Provides: {"macro_7229"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for usages of `str.trim().split(\"\\n\")` and `str.trim().split(\"\\r\\n\")`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Hard-coding the line endings makes the code less compatible. `str.lines` should be used instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \"some\\ntext\\nwith\\nnewlines\\n\".trim().split('\\n');"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " \"some\\ntext\\nwith\\nnewlines\\n\".lines();"] # [doc = " ```"] # [doc = ""] # [doc = " ### Known Problems"] # [doc = ""] # [doc = " This lint cannot detect if the split is intentionally restricted to a single type of newline (`\"\\n\"` or"] # [doc = " `\"\\r\\n\"`), for example during the parsing of a specific file format in which precisely one newline type is"] # [doc = " valid."] # [clippy :: version = "1.77.0"] pub STR_SPLIT_AT_NEWLINE , pedantic , "splitting a trimmed string at hard-coded newlines" }
};
}
