// Generated macro for macro_10624 (macro)
macro_rules! Depcrate_unneeded_struct_patternmacro_10624 {
() => {
// Module: crate::unneeded_struct_pattern
// Provides: {"macro_10624"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for struct patterns that match against unit variant."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Struct pattern `{ }` or `{ .. }` is not needed for unit variant."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " match Some(42) {"] # [doc = "     Some(v) => v,"] # [doc = "     None { .. } => 0,"] # [doc = " };"] # [doc = " // Or"] # [doc = " match Some(42) {"] # [doc = "     Some(v) => v,"] # [doc = "     None { } => 0,"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " match Some(42) {"] # [doc = "     Some(v) => v,"] # [doc = "     None => 0,"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub UNNEEDED_STRUCT_PATTERN , style , "using struct pattern to match against unit variant" }
};
}
