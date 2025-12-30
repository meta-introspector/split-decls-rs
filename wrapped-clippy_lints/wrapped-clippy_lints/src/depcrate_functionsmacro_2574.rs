// Generated macro for macro_2574 (macro)
macro_rules! Depcrate_functionsmacro_2574 {
() => {
// Module: crate::functions
// Provides: {"macro_2574"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for a `#[must_use]` attribute without"] # [doc = " further information on functions and methods that return a type already"] # [doc = " marked as `#[must_use]`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The attribute isn't needed. Not using the result"] # [doc = " will already be reported. Alternatively, one can add some text to the"] # [doc = " attribute to improve the lint message."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " #[must_use]"] # [doc = " fn double_must_use() -> Result<(), ()> {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub DOUBLE_MUST_USE , style , "`#[must_use]` attribute on a `#[must_use]`-returning function / method" }
};
}
