// Generated macro for macro_2781 (macro)
macro_rules! Depcrate_implied_bounds_in_implsmacro_2781 {
() => {
// Module: crate::implied_bounds_in_impls
// Provides: {"macro_2781"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for bounds in `impl Trait` in return position that are implied by other bounds."] # [doc = " This can happen when a trait is specified that another trait already has as a supertrait"] # [doc = " (e.g. `fn() -> impl Deref + DerefMut<Target = i32>` has an unnecessary `Deref` bound,"] # [doc = " because `Deref` is a supertrait of `DerefMut`)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Specifying more bounds than necessary adds needless complexity for the reader."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " This lint does not check for implied bounds transitively. Meaning that"] # [doc = " it doesn't check for implied bounds from supertraits of supertraits"] # [doc = " (e.g. `trait A {} trait B: A {} trait C: B {}`, then having an `fn() -> impl A + C`)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::ops::{Deref,DerefMut};"] # [doc = " fn f() -> impl Deref<Target = i32> + DerefMut<Target = i32> {"] # [doc = " //             ^^^^^^^^^^^^^^^^^^^ unnecessary bound, already implied by the `DerefMut` trait bound"] # [doc = "     Box::new(123)"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::ops::{Deref,DerefMut};"] # [doc = " fn f() -> impl DerefMut<Target = i32> {"] # [doc = "     Box::new(123)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub IMPLIED_BOUNDS_IN_IMPLS , complexity , "specifying bounds that are implied by other bounds in `impl Trait` type" }
};
}
