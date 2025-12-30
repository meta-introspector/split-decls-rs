// Generated macro for macro_7073 (macro)
macro_rules! Depcrate_methodsmacro_7073 {
() => {
// Module: crate::methods
// Provides: {"macro_7073"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `as_str()` on a `String` chained with a method available on the `String` itself."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `as_str()` conversion is pointless and can be removed for simplicity and cleanliness."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let owned_string = \"This is a string\".to_owned();"] # [doc = " owned_string.as_str().as_bytes()"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let owned_string = \"This is a string\".to_owned();"] # [doc = " owned_string.as_bytes()"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub REDUNDANT_AS_STR , complexity , "`as_str` used to call a method on `str` that is also available on `String`" }
};
}
