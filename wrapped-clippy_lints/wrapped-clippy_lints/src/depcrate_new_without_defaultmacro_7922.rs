// Generated macro for macro_7922 (macro)
macro_rules! Depcrate_new_without_defaultmacro_7922 {
() => {
// Module: crate::new_without_default
// Provides: {"macro_7922"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for public types with a `pub fn new() -> Self` method and no"] # [doc = " implementation of"] # [doc = " [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The user might expect to be able to use"] # [doc = " [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) as the"] # [doc = " type can be constructed without arguments."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " pub struct Foo(Bar);"] # [doc = ""] # [doc = " impl Foo {"] # [doc = "     pub fn new() -> Self {"] # [doc = "         Foo(Bar::new())"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " To fix the lint, add a `Default` implementation that delegates to `new`:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " pub struct Foo(Bar);"] # [doc = ""] # [doc = " impl Default for Foo {"] # [doc = "     fn default() -> Self {"] # [doc = "         Foo::new()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEW_WITHOUT_DEFAULT , style , "`pub fn new() -> Self` method without `Default` implementation" }
};
}
