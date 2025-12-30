// Generated macro for macro_10102 (macro)
macro_rules! Depcrate_transmutemacro_10102 {
() => {
// Module: crate::transmute
// Provides: {"macro_10102"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes between a type `T` and `*T`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's easy to mistakenly transmute between a type and a"] # [doc = " pointer to that type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " core::intrinsics::transmute(t) // where the result type is the same as"] # [doc = "                                // `*t` or `&t`'s"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CROSSPOINTER_TRANSMUTE , suspicious , "transmutes that have to or from types that are a pointer to the other" }
};
}
