// Generated macro for macro_6968 (macro)
macro_rules! Depcrate_methodsmacro_6968 {
() => {
// Module: crate::methods
// Provides: {"macro_6968"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usages of the following functions with an argument that constructs a default value"] # [doc = " (e.g., `Default::default` or `String::new`):"] # [doc = " - `unwrap_or`"] # [doc = " - `unwrap_or_else`"] # [doc = " - `or_insert`"] # [doc = " - `or_insert_with`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. Using `unwrap_or_default` in place of `unwrap_or`/`unwrap_or_else`, or `or_default`"] # [doc = " in place of `or_insert`/`or_insert_with`, is simpler and more concise."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " In some cases, the argument of `unwrap_or`, etc. is needed for type inference. The lint uses a"] # [doc = " heuristic to try to identify such cases. However, the heuristic can produce false negatives."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " # let x = Some(1);"] # [doc = " # let mut map = std::collections::HashMap::<u64, String>::new();"] # [doc = " x.unwrap_or(Default::default());"] # [doc = " map.entry(42).or_insert_with(String::new);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = Some(1);"] # [doc = " # let mut map = std::collections::HashMap::<u64, String>::new();"] # [doc = " x.unwrap_or_default();"] # [doc = " map.entry(42).or_default();"] # [doc = " ```"] # [clippy :: version = "1.56.0"] pub UNWRAP_OR_DEFAULT , style , "using `.unwrap_or`, etc. with an argument that constructs a default value" }
};
}
