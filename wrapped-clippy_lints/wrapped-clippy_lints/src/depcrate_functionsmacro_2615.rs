// Generated macro for macro_2615 (macro)
macro_rules! Depcrate_functionsmacro_2615 {
() => {
// Module: crate::functions
// Provides: {"macro_2615"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints when `impl Trait` is being used in a function's parameters."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Turbofish syntax (`::<>`) cannot be used to specify the type of an `impl Trait` parameter,"] # [doc = " making `impl Trait` less powerful. Readability may also be a factor."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " trait MyTrait {}"] # [doc = " fn foo(a: impl MyTrait) {"] # [doc = " \t// [...]"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " trait MyTrait {}"] # [doc = " fn foo<T: MyTrait>(a: T) {"] # [doc = " \t// [...]"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub IMPL_TRAIT_IN_PARAMS , restriction , "`impl Trait` is used in the function's parameters" }
};
}
