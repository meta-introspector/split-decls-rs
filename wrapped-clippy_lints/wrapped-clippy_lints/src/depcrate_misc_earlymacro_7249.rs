// Generated macro for macro_7249 (macro)
macro_rules! Depcrate_misc_earlymacro_7249 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7249"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for structure field patterns bound to wildcards."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Using `..` instead is shorter and leaves the focus on"] # [doc = " the fields that are actually bound."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct Foo {"] # [doc = " #     a: i32,"] # [doc = " #     b: i32,"] # [doc = " #     c: i32,"] # [doc = " # }"] # [doc = " let f = Foo { a: 0, b: 0, c: 0 };"] # [doc = ""] # [doc = " match f {"] # [doc = "     Foo { a: _, b: 0, .. } => {},"] # [doc = "     Foo { a: _, b: _, c: _ } => {},"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # struct Foo {"] # [doc = " #     a: i32,"] # [doc = " #     b: i32,"] # [doc = " #     c: i32,"] # [doc = " # }"] # [doc = " let f = Foo { a: 0, b: 0, c: 0 };"] # [doc = ""] # [doc = " match f {"] # [doc = "     Foo { b: 0, .. } => {},"] # [doc = "     Foo { .. } => {},"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNNEEDED_FIELD_PATTERN , restriction , "struct fields bound to a wildcard instead of using `..`" }
};
}
