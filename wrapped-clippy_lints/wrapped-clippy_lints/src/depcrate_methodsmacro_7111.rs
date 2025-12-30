// Generated macro for macro_7111 (macro)
macro_rules! Depcrate_methodsmacro_7111 {
() => {
// Module: crate::methods
// Provides: {"macro_7111"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for methods that should live in a trait"] # [doc = " implementation of a `std` trait (see [llogiq's blog"] # [doc = " post](http://llogiq.github.io/2015/07/30/traits.html) for further"] # [doc = " information) instead of an inherent implementation."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Implementing the traits improve ergonomics for users of"] # [doc = " the code, often with very little cost. Also people seeing a `mul(...)`"] # [doc = " method"] # [doc = " may expect `*` to work equally, so you should have good reason to disappoint"] # [doc = " them."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct X;"] # [doc = " impl X {"] # [doc = "     fn add(&self, other: &X) -> X {"] # [doc = "         // .."] # [doc = " # X"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SHOULD_IMPLEMENT_TRAIT , style , "defining a method that should be implementing a std trait" }
};
}
