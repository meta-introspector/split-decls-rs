// Generated macro for macro_10331 (macro)
macro_rules! Depcrate_undocumented_unsafe_blocksmacro_10331 {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"macro_10331"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `// SAFETY: ` comments on safe code."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Safe code has no safety requirements, so there is no need to"] # [doc = " describe safety invariants."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::ptr::NonNull;"] # [doc = " let a = &mut 42;"] # [doc = ""] # [doc = " // SAFETY: references are guaranteed to be non-null."] # [doc = " let ptr = NonNull::new(a).unwrap();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::ptr::NonNull;"] # [doc = " let a = &mut 42;"] # [doc = ""] # [doc = " let ptr = NonNull::new(a).unwrap();"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub UNNECESSARY_SAFETY_COMMENT , restriction , "annotating safe code with a safety comment" }
};
}
