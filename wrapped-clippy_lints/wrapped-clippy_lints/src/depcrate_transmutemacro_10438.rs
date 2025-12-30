// Generated macro for macro_10438 (macro)
macro_rules! Depcrate_transmutemacro_10438 {
() => {
// Module: crate::transmute
// Provides: {"macro_10438"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes to the original type of the object"] # [doc = " and transmutes that could be a cast."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. The code tricks people into thinking that"] # [doc = " something complex is going on."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " core::intrinsics::transmute(t); // where the result type is the same as `t`'s"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USELESS_TRANSMUTE , complexity , "transmutes that have the same to and from types or could be a cast/coercion" }
};
}
