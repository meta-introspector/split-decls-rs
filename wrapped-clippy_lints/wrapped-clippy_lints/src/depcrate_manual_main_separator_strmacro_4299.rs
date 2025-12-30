// Generated macro for macro_4299 (macro)
macro_rules! Depcrate_manual_main_separator_strmacro_4299 {
() => {
// Module: crate::manual_main_separator_str
// Provides: {"macro_4299"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for references on `std::path::MAIN_SEPARATOR.to_string()` used"] # [doc = " to build a `&str`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " There exists a `std::path::MAIN_SEPARATOR_STR` which does not require"] # [doc = " an extra memory allocation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let s: &str = &std::path::MAIN_SEPARATOR.to_string();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let s: &str = std::path::MAIN_SEPARATOR_STR;"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub MANUAL_MAIN_SEPARATOR_STR , complexity , "`&std::path::MAIN_SEPARATOR.to_string()` can be replaced by `std::path::MAIN_SEPARATOR_STR`" }
};
}
