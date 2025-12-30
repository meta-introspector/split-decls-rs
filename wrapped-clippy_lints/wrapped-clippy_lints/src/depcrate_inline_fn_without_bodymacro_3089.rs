// Generated macro for macro_3089 (macro)
macro_rules! Depcrate_inline_fn_without_bodymacro_3089 {
() => {
// Module: crate::inline_fn_without_body
// Provides: {"macro_3089"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[inline]` on trait methods without bodies"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Only implementations of trait methods may be inlined."] # [doc = " The inline attribute is ignored for trait methods without bodies."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " trait Animal {"] # [doc = "     #[inline]"] # [doc = "     fn name(&self) -> &'static str;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INLINE_FN_WITHOUT_BODY , correctness , "use of `#[inline]` on trait methods without bodies" }
};
}
