// Generated macro for Callable (struct)
macro_rules! Depcrate_util_callableCallable {
() => {
// Module: crate::util::callable
// Provides: {"Callable"}
// Dependencies: {}
# [doc = " Either a path or a closure."] # [doc = ""] # [doc = " This type is useful for options that historically took a path,"] # [doc = " e.g. `#[darling(with = ...)]` or `#[serde(skip_serializing_if = ...)]`"] # [doc = " and now want to also allow using a closure to avoid needing a separate"] # [doc = " function declaration."] # [doc = ""] # [doc = " In `darling`, this value is wrapped in [`core::convert::identity`] before usage;"] # [doc = " this allows treatment of the closure and path cases as equivalent, and prevents"] # [doc = " a closure from accessing locals in the generated code."] # [derive (Debug , Clone)] pub struct Callable { # [doc = " The callable"] call : Expr , }
};
}
