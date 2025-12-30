// Generated macro for macro_10898 (macro)
macro_rules! Depcrate_use_selfmacro_10898 {
() => {
// Module: crate::use_self
// Provides: {"macro_10898"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary repetition of structure name when a"] # [doc = " replacement with `Self` is applicable."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unnecessary repetition. Mixed use of `Self` and struct"] # [doc = " name"] # [doc = " feels inconsistent."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " - Unaddressed false negative in fn bodies of trait implementations"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo;"] # [doc = " impl Foo {"] # [doc = "     fn new() -> Foo {"] # [doc = "         Foo {}"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " could be"] # [doc = " ```no_run"] # [doc = " struct Foo;"] # [doc = " impl Foo {"] # [doc = "     fn new() -> Self {"] # [doc = "         Self {}"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USE_SELF , nursery , "unnecessary structure name repetition whereas `Self` is applicable" }
};
}
