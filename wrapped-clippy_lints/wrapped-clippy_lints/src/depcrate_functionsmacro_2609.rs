// Generated macro for macro_2609 (macro)
macro_rules! Depcrate_functionsmacro_2609 {
() => {
// Module: crate::functions
// Provides: {"macro_2609"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for a `#[must_use]` attribute on"] # [doc = " unit-returning functions and methods."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unit values are useless. The attribute is likely"] # [doc = " a remnant of a refactoring that removed the return type."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " #[must_use]"] # [doc = " fn useless() { }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub MUST_USE_UNIT , style , "`#[must_use]` attribute on a unit-returning function / method" }
};
}
