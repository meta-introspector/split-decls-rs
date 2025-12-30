// Generated macro for macro_11037 (macro)
macro_rules! Depcrate_wildcard_importsmacro_11037 {
() => {
// Module: crate::wildcard_imports
// Provides: {"macro_11037"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `use Enum::*`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is usually better style to use the prefixed name of"] # [doc = " an enumeration variant, rather than importing variants."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Old-style enumerations that prefix the variants are"] # [doc = " still around."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::cmp::Ordering::*;"] # [doc = ""] # [doc = " # fn foo(_: std::cmp::Ordering) {}"] # [doc = " foo(Less);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::cmp::Ordering;"] # [doc = ""] # [doc = " # fn foo(_: Ordering) {}"] # [doc = " foo(Ordering::Less)"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ENUM_GLOB_USE , pedantic , "use items that import all variants of an enum" }
};
}
