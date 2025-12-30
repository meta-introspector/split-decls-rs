// Generated macro for macro_7169 (macro)
macro_rules! Depcrate_methodsmacro_7169 {
() => {
// Module: crate::methods
// Provides: {"macro_7169"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of `_.to_owned()`, `vec.to_vec()`, or similar when calling `_.clone()` would be clearer."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These methods do the same thing as `_.clone()` but may be confusing as"] # [doc = " to why we are calling `to_vec` on something that is already a `Vec` or calling `to_owned` on something that is already owned."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = vec![1, 2, 3];"] # [doc = " let b = a.to_vec();"] # [doc = " let c = a.to_owned();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = vec![1, 2, 3];"] # [doc = " let b = a.clone();"] # [doc = " let c = a.clone();"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub IMPLICIT_CLONE , pedantic , "implicitly cloning a value by invoking a function on its dereferenced type" }
};
}
