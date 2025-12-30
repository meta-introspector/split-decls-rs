// Generated macro for macro_7190 (macro)
macro_rules! Depcrate_methodsmacro_7190 {
() => {
// Module: crate::methods
// Provides: {"macro_7190"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `map(|x| x.clone())` or"] # [doc = " dereferencing closures for `Copy` types, on `Iterator` or `Option`,"] # [doc = " and suggests `cloned()` or `copied()` instead"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = vec![42, 43];"] # [doc = " let y = x.iter();"] # [doc = " let z = y.map(|i| *i);"] # [doc = " ```"] # [doc = ""] # [doc = " The correct use would be:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let x = vec![42, 43];"] # [doc = " let y = x.iter();"] # [doc = " let z = y.cloned();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MAP_CLONE , style , "using `iterator.map(|x| x.clone())`, or dereferencing closures for `Copy` types" }
};
}
