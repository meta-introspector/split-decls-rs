// Generated macro for macro_2824 (macro)
macro_rules! Depcrate_inconsistent_struct_constructormacro_2824 {
() => {
// Module: crate::inconsistent_struct_constructor
// Provides: {"macro_2824"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for struct constructors where the order of the field"] # [doc = " init in the constructor is inconsistent with the order in the"] # [doc = " struct definition."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since the order of fields in a constructor doesn't affect the"] # [doc = " resulted instance as the below example indicates,"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #[derive(Debug, PartialEq, Eq)]"] # [doc = " struct Foo {"] # [doc = "     x: i32,"] # [doc = "     y: i32,"] # [doc = " }"] # [doc = " let x = 1;"] # [doc = " let y = 2;"] # [doc = ""] # [doc = " // This assertion never fails:"] # [doc = " assert_eq!(Foo { x, y }, Foo { y, x });"] # [doc = " ```"] # [doc = ""] # [doc = " inconsistent order can be confusing and decreases readability and consistency."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo {"] # [doc = "     x: i32,"] # [doc = "     y: i32,"] # [doc = " }"] # [doc = " let x = 1;"] # [doc = " let y = 2;"] # [doc = ""] # [doc = " Foo { y, x };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # struct Foo {"] # [doc = " #     x: i32,"] # [doc = " #     y: i32,"] # [doc = " # }"] # [doc = " # let x = 1;"] # [doc = " # let y = 2;"] # [doc = " Foo { x, y };"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub INCONSISTENT_STRUCT_CONSTRUCTOR , pedantic , "the order of the field init is inconsistent with the order in the struct definition" }
};
}
