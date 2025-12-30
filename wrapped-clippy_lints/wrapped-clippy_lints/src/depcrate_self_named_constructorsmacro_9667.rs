// Generated macro for macro_9667 (macro)
macro_rules! Depcrate_self_named_constructorsmacro_9667 {
() => {
// Module: crate::self_named_constructors
// Provides: {"macro_9667"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns when constructors have the same name as their types."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Repeating the name of the type is redundant."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " struct Foo {}"] # [doc = ""] # [doc = " impl Foo {"] # [doc = "     pub fn foo() -> Foo {"] # [doc = "         Foo {}"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " struct Foo {}"] # [doc = ""] # [doc = " impl Foo {"] # [doc = "     pub fn new() -> Foo {"] # [doc = "         Foo {}"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub SELF_NAMED_CONSTRUCTORS , style , "method should not have the same name as the type it is implemented for" }
};
}
