// Generated macro for macro_3357 (macro)
macro_rules! Depcrate_lifetimesmacro_3357 {
() => {
// Module: crate::lifetimes
// Provides: {"macro_3357"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for lifetime annotations which can be removed by"] # [doc = " relying on lifetime elision."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The additional lifetimes make the code look more"] # [doc = " complicated, while there is nothing out of the ordinary going on. Removing"] # [doc = " them leads to more readable code."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint ignores functions with `where` clauses that reference"] # [doc = " lifetimes to prevent false positives."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // Unnecessary lifetime annotations"] # [doc = " fn in_and_out<'a>(x: &'a u8, y: u8) -> &'a u8 {"] # [doc = "     x"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn elided(x: &u8, y: u8) -> &u8 {"] # [doc = "     x"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_LIFETIMES , complexity , "using explicit lifetimes for references in function arguments when elision rules \
     would allow omitting them" }
};
}
