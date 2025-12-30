// Generated macro for macro_1770 (macro)
macro_rules! Depcrate_docmacro_1770 {
() => {
// Module: crate::doc
// Provides: {"macro_1770"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the doc comments of publicly visible"] # [doc = " safe functions and traits and warns if there is a `# Safety` section."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Safe functions and traits are safe to implement and therefore do not"] # [doc = " need to describe safety preconditions that users are required to uphold."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = "# type Universe = ();"] # [doc = " /// # Safety"] # [doc = " ///"] # [doc = " /// This function should not be called before the horsemen are ready."] # [doc = " pub fn start_apocalypse_but_safely(u: &mut Universe) {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The function is safe, so there shouldn't be any preconditions"] # [doc = " that have to be explained for safety reasons."] # [doc = ""] # [doc = " ```no_run"] # [doc = "# type Universe = ();"] # [doc = " /// This function should really be documented"] # [doc = " pub fn start_apocalypse(u: &mut Universe) {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub UNNECESSARY_SAFETY_DOC , restriction , "`pub fn` or `pub trait` with `# Safety` docs" }
};
}
