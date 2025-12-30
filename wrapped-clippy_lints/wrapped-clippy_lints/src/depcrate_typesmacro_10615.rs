// Generated macro for macro_10615 (macro)
macro_rules! Depcrate_typesmacro_10615 {
() => {
// Module: crate::types
// Provides: {"macro_10615"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `&Box<T>` anywhere in the code."] # [doc = " Check the [Box documentation](https://doc.rust-lang.org/std/boxed/index.html) for more information."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " A `&Box<T>` parameter requires the function caller to box `T` first before passing it to a function."] # [doc = " Using `&T` defines a concrete type for the parameter and generalizes the function, this would also"] # [doc = " auto-deref to `&T` at the function call site if passed a `&Box<T>`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " fn foo(bar: &Box<T>) { ... }"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " fn foo(bar: &T) { ... }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub BORROWED_BOX , complexity , "a borrow of a boxed type" }
};
}
