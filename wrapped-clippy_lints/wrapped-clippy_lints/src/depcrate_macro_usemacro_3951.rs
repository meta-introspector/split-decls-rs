// Generated macro for macro_3951 (macro)
macro_rules! Depcrate_macro_usemacro_3951 {
() => {
// Module: crate::macro_use
// Provides: {"macro_3951"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[macro_use] use...`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since the Rust 2018 edition you can import"] # [doc = " macro's directly, this is considered idiomatic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " #[macro_use]"] # [doc = " extern crate some_crate;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     some_macro!();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use some_crate::some_macro;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     some_macro!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub MACRO_USE_IMPORTS , pedantic , "#[macro_use] is no longer needed" }
};
}
