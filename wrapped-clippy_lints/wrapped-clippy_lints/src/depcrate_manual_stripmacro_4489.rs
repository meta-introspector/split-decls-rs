// Generated macro for macro_4489 (macro)
macro_rules! Depcrate_manual_stripmacro_4489 {
() => {
// Module: crate::manual_strip
// Provides: {"macro_4489"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Suggests using `strip_{prefix,suffix}` over `str::{starts,ends}_with` and slicing using"] # [doc = " the pattern's length."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `str:strip_{prefix,suffix}` is safer and may have better performance as there is no"] # [doc = " slicing which may panic and the compiler does not need to insert this panic code. It is"] # [doc = " also sometimes more readable as it removes the need for duplicating or storing the pattern"] # [doc = " used by `str::{starts,ends}_with` and in the slicing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let s = \"hello, world!\";"] # [doc = " if s.starts_with(\"hello, \") {"] # [doc = "     assert_eq!(s[\"hello, \".len()..].to_uppercase(), \"WORLD!\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let s = \"hello, world!\";"] # [doc = " if let Some(end) = s.strip_prefix(\"hello, \") {"] # [doc = "     assert_eq!(end.to_uppercase(), \"WORLD!\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub MANUAL_STRIP , complexity , "suggests using `strip_{prefix,suffix}` over `str::{starts,ends}_with` and slicing" }
};
}
