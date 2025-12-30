// Generated macro for macro_7390 (macro)
macro_rules! Depcrate_misc_earlymacro_7390 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7390"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if literal suffixes are separated by an underscore."] # [doc = " To enforce separated literal suffix style,"] # [doc = " see the `unseparated_literal_suffix` lint."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Suffix style should be consistent."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let _ ="] # [doc = " 123832_i32"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let _ ="] # [doc = " 123832i32"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub SEPARATED_LITERAL_SUFFIX , restriction , "literals whose suffix is separated by an underscore" }
};
}
