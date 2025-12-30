// Generated macro for macro_9513 (macro)
macro_rules! Depcrate_single_char_lifetime_namesmacro_9513 {
() => {
// Module: crate::single_char_lifetime_names
// Provides: {"macro_9513"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for lifetimes with names which are one character"] # [doc = " long."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " A single character is likely not enough to express the"] # [doc = " purpose of a lifetime. Using a longer name can make code"] # [doc = " easier to understand."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Rust programmers and learning resources tend to use single"] # [doc = " character lifetimes, so this lint is at odds with the"] # [doc = " ecosystem at large. In addition, the lifetime's purpose may"] # [doc = " be obvious or, rarely, expressible in one character."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct DiagnosticCtx<'a> {"] # [doc = "     source: &'a str,"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct DiagnosticCtx<'src> {"] # [doc = "     source: &'src str,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.60.0"] pub SINGLE_CHAR_LIFETIME_NAMES , restriction , "warns against single-character lifetime names" }
};
}
