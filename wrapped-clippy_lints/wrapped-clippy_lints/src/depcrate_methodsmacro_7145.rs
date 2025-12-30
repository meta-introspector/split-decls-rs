// Generated macro for macro_7145 (macro)
macro_rules! Depcrate_methodsmacro_7145 {
() => {
// Module: crate::methods
// Provides: {"macro_7145"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of `.extend(s.chars())` where s is a"] # [doc = " `&str` or `String`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.push_str(s)` is clearer"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let abc = \"abc\";"] # [doc = " let def = String::from(\"def\");"] # [doc = " let mut s = String::new();"] # [doc = " s.extend(abc.chars());"] # [doc = " s.extend(def.chars());"] # [doc = " ```"] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " let abc = \"abc\";"] # [doc = " let def = String::from(\"def\");"] # [doc = " let mut s = String::new();"] # [doc = " s.push_str(abc);"] # [doc = " s.push_str(&def);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub STRING_EXTEND_CHARS , style , "using `x.extend(s.chars())` where s is a `&str` or `String`" }
};
}
