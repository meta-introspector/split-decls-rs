// Generated macro for macro_7161 (macro)
macro_rules! Depcrate_methodsmacro_7161 {
() => {
// Module: crate::methods
// Provides: {"macro_7161"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns when using `push_str`/`insert_str` with a single-character string literal"] # [doc = " where `push`/`insert` with a `char` would work fine."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's less clear that we are pushing a single character."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let mut string = String::new();"] # [doc = " string.insert_str(0, \"R\");"] # [doc = " string.push_str(\"R\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let mut string = String::new();"] # [doc = " string.insert(0, 'R');"] # [doc = " string.push('R');"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub SINGLE_CHAR_ADD_STR , style , "`push_str()` or `insert_str()` used with a single-character string literal as parameter" }
};
}
