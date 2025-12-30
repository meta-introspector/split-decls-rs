// Generated macro for macro_9223 (macro)
macro_rules! Depcrate_ref_patternsmacro_9223 {
() => {
// Module: crate::ref_patterns
// Provides: {"macro_9223"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usages of the `ref` keyword."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The `ref` keyword can be confusing for people unfamiliar with it, and often"] # [doc = " it is more concise to use `&` instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let opt = Some(5);"] # [doc = " if let Some(ref foo) = opt {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let opt = Some(5);"] # [doc = " if let Some(foo) = &opt {}"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub REF_PATTERNS , restriction , "use of a ref pattern, e.g. Some(ref value)" }
};
}
