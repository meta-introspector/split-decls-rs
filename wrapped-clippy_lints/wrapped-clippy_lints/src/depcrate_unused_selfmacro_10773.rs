// Generated macro for macro_10773 (macro)
macro_rules! Depcrate_unused_selfmacro_10773 {
() => {
// Module: crate::unused_self
// Provides: {"macro_10773"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks methods that contain a `self` argument but don't use it"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It may be clearer to define the method as an associated function instead"] # [doc = " of an instance method if it doesn't require `self`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " struct A;"] # [doc = " impl A {"] # [doc = "     fn method(&self) {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " struct A;"] # [doc = " impl A {"] # [doc = "     fn method() {}"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub UNUSED_SELF , pedantic , "methods that contain a `self` argument but don't use it" }
};
}
