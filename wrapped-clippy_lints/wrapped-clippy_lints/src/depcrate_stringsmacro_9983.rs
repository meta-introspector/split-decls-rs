// Generated macro for macro_9983 (macro)
macro_rules! Depcrate_stringsmacro_9983 {
() => {
// Module: crate::strings
// Provides: {"macro_9983"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint checks for `.to_string()` method calls on values of type `&str`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The `to_string` method is also used on other types to convert them to a string."] # [doc = " When called on a `&str` it turns the `&str` into the owned variant `String`, which can be"] # [doc = " more specifically expressed with `.to_owned()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = \"str\".to_string();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = \"str\".to_owned();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub STR_TO_STRING , restriction , "using `to_string()` on a `&str`, which should be `to_owned()`" }
};
}
