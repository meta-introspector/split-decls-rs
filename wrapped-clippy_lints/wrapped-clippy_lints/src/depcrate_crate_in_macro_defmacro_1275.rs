// Generated macro for macro_1275 (macro)
macro_rules! Depcrate_crate_in_macro_defmacro_1275 {
() => {
// Module: crate::crate_in_macro_def
// Provides: {"macro_1275"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `crate` as opposed to `$crate` in a macro definition."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `crate` refers to the macro call's crate, whereas `$crate` refers to the macro definition's"] # [doc = " crate. Rarely is the former intended. See:"] # [doc = " https://doc.rust-lang.org/reference/macros-by-example.html#hygiene"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[macro_export]"] # [doc = " macro_rules! print_message {"] # [doc = "     () => {"] # [doc = "         println!(\"{}\", crate::MESSAGE);"] # [doc = "     };"] # [doc = " }"] # [doc = " pub const MESSAGE: &str = \"Hello!\";"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[macro_export]"] # [doc = " macro_rules! print_message {"] # [doc = "     () => {"] # [doc = "         println!(\"{}\", $crate::MESSAGE);"] # [doc = "     };"] # [doc = " }"] # [doc = " pub const MESSAGE: &str = \"Hello!\";"] # [doc = " ```"] # [doc = ""] # [doc = " Note that if the use of `crate` is intentional, an `allow` attribute can be applied to the"] # [doc = " macro definition, e.g.:"] # [doc = " ```rust,ignore"] # [doc = " #[allow(clippy::crate_in_macro_def)]"] # [doc = " macro_rules! ok { ... crate::foo ... }"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub CRATE_IN_MACRO_DEF , suspicious , "using `crate` in a macro definition" }
};
}
