// Generated macro for macro_1174 (macro)
macro_rules! Depcrate_redundant_semicolonmacro_1174 {
() => {
// Module: crate::redundant_semicolon
// Provides: {"macro_1174"}
// Dependencies: {}
declare_lint ! { # [doc = " The `redundant_semicolons` lint detects unnecessary trailing"] # [doc = " semicolons."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let _ = 123;;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Extra semicolons are not needed, and may be removed to avoid confusion"] # [doc = " and visual clutter."] pub REDUNDANT_SEMICOLONS , Warn , "detects unnecessary trailing semicolons" }
};
}
