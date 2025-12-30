// Generated macro for macro_8963 (macro)
macro_rules! Depcrate_raw_stringsmacro_8963 {
() => {
// Module: crate::raw_strings
// Provides: {"macro_8963"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for raw string literals where a string literal can be used instead."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " For consistent style by using simpler string literals whenever possible."] # [doc = ""] # [doc = " However, there are many cases where using a raw string literal is more"] # [doc = " idiomatic than a string literal, so it's opt-in."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let r = r\"Hello, world!\";"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let r = \"Hello, world!\";"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub NEEDLESS_RAW_STRINGS , restriction , "suggests using a string literal when a raw string literal is unnecessary" }
};
}
