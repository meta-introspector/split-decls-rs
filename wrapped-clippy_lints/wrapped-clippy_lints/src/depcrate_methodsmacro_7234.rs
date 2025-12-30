// Generated macro for macro_7234 (macro)
macro_rules! Depcrate_methodsmacro_7234 {
() => {
// Module: crate::methods
// Provides: {"macro_7234"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks the usage of `.get().is_some()` or `.get().is_none()` on std map types."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can be done in one call with `.contains()`/`.contains_key()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " let s: HashSet<String> = HashSet::new();"] # [doc = " if s.get(\"a\").is_some() {"] # [doc = "     // code"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " let s: HashSet<String> = HashSet::new();"] # [doc = " if s.contains(\"a\") {"] # [doc = "     // code"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub UNNECESSARY_GET_THEN_CHECK , suspicious , "calling `.get().is_some()` or `.get().is_none()` instead of `.contains()` or `.contains_key()`" }
};
}
