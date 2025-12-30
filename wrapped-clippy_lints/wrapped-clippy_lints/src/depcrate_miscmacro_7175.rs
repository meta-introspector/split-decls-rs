// Generated macro for macro_7175 (macro)
macro_rules! Depcrate_miscmacro_7175 {
() => {
// Module: crate::misc
// Provides: {"macro_7175"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for function arguments and let bindings denoted as"] # [doc = " `ref`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `ref` declaration makes the function take an owned"] # [doc = " value, but turns the argument into a reference (which means that the value"] # [doc = " is destroyed when exiting the function). This adds not much value: either"] # [doc = " take a reference type, or take an owned value and create references in the"] # [doc = " body."] # [doc = ""] # [doc = " For let bindings, `let x = &foo;` is preferred over `let ref x = foo`. The"] # [doc = " type of `x` is more obvious with the former."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the argument is dereferenced within the function,"] # [doc = " removing the `ref` will lead to errors. This can be fixed by removing the"] # [doc = " dereferences, e.g., changing `*x` to `x` within the function."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(ref _x: u8) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(_x: &u8) {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TOPLEVEL_REF_ARG , style , "an entire binding declared as `ref`, in a function argument or a `let` statement" }
};
}
