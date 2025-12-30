// Generated macro for macro_7858 (macro)
macro_rules! Depcrate_needless_elsemacro_7858 {
() => {
// Module: crate::needless_else
// Provides: {"macro_7858"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty `else` branches."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " An empty else branch does nothing and can be removed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = "# fn check() -> bool { true }"] # [doc = " if check() {"] # [doc = "     println!(\"Check successful!\");"] # [doc = " } else {"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = "# fn check() -> bool { true }"] # [doc = " if check() {"] # [doc = "     println!(\"Check successful!\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub NEEDLESS_ELSE , style , "empty else branch" }
};
}
