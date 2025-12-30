// Generated macro for macro_11060 (macro)
macro_rules! Depcrate_writemacro_11060 {
() => {
// Module: crate::write
// Provides: {"macro_11060"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns when you use `println!(\"\")` to"] # [doc = " print a newline."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " You should use `println!()`, which is simpler."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " println!(\"\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " println!();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PRINTLN_EMPTY_STRING , style , "using `println!(\"\")` with an empty string" }
};
}
