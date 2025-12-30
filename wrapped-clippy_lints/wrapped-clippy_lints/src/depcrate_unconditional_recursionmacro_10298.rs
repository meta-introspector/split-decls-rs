// Generated macro for macro_10298 (macro)
macro_rules! Depcrate_unconditional_recursionmacro_10298 {
() => {
// Module: crate::unconditional_recursion
// Provides: {"macro_10298"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks that there isn't an infinite recursion in trait"] # [doc = " implementations."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Infinite recursion in trait implementation will either cause crashes"] # [doc = " or result in an infinite loop, and it is hard to detect."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum Foo {"] # [doc = "     A,"] # [doc = "     B,"] # [doc = " }"] # [doc = ""] # [doc = " impl PartialEq for Foo {"] # [doc = "     fn eq(&self, other: &Self) -> bool {"] # [doc = "         self == other // bad!"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #[derive(PartialEq)]"] # [doc = " enum Foo {"] # [doc = "     A,"] # [doc = "     B,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " As an alternative, rewrite the logic without recursion:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " enum Foo {"] # [doc = "     A,"] # [doc = "     B,"] # [doc = " }"] # [doc = ""] # [doc = " impl PartialEq for Foo {"] # [doc = "     fn eq(&self, other: &Self) -> bool {"] # [doc = "         matches!((self, other), (Foo::A, Foo::A) | (Foo::B, Foo::B))"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub UNCONDITIONAL_RECURSION , suspicious , "detect unconditional recursion in some traits implementation" }
};
}
