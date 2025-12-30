// Generated macro for macro_3423 (macro)
macro_rules! Depcrate_lifetimesmacro_3423 {
() => {
// Module: crate::lifetimes
// Provides: {"macro_3423"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for lifetimes in generics that are never used"] # [doc = " anywhere else."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The additional lifetimes make the code look more"] # [doc = " complicated, while there is nothing out of the ordinary going on. Removing"] # [doc = " them leads to more readable code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // unnecessary lifetimes"] # [doc = " fn unused_lifetime<'a>(x: u8) {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn no_lifetime(x: u8) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXTRA_UNUSED_LIFETIMES , complexity , "unused lifetimes in function definitions" }
};
}
