// Generated macro for macro_1763 (macro)
macro_rules! Depcrate_docmacro_1763 {
() => {
// Module: crate::doc
// Provides: {"macro_1763"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the doc comments of publicly visible"] # [doc = " unsafe functions and warns if there is no `# Safety` section."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unsafe functions should document their safety"] # [doc = " preconditions, so that users can be sure they are using them safely."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = "# type Universe = ();"] # [doc = " /// This function should really be documented"] # [doc = " pub unsafe fn start_apocalypse(u: &mut Universe) {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " At least write a line about safety:"] # [doc = ""] # [doc = " ```no_run"] # [doc = "# type Universe = ();"] # [doc = " /// # Safety"] # [doc = " ///"] # [doc = " /// This function should not be called before the horsemen are ready."] # [doc = " pub unsafe fn start_apocalypse(u: &mut Universe) {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.39.0"] pub MISSING_SAFETY_DOC , style , "`pub unsafe fn` without `# Safety` docs" }
};
}
