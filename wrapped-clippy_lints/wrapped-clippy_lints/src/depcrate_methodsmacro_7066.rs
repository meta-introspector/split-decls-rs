// Generated macro for macro_7066 (macro)
macro_rules! Depcrate_methodsmacro_7066 {
() => {
// Module: crate::methods
// Provides: {"macro_7066"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `<string_lit>.chars().any(|i| i == c)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's significantly slower than using a pattern instead, like"] # [doc = " `matches!(c, '\\\\' | '.' | '+')`."] # [doc = ""] # [doc = " Despite this being faster, this is not `perf` as this is pretty common, and is a rather nice"] # [doc = " way to check if a `char` is any in a set. In any case, this `restriction` lint is available"] # [doc = " for situations where that additional performance is absolutely necessary."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let c = 'c';"] # [doc = " \"\\\\.+*?()|[]{}^$#&-~\".chars().any(|x| x == c);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let c = 'c';"] # [doc = " matches!(c, '\\\\' | '.' | '+' | '*' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' | '#' | '&' | '-' | '~');"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub STRING_LIT_CHARS_ANY , restriction , "checks for `<string_lit>.chars().any(|i| i == c)`" }
};
}
