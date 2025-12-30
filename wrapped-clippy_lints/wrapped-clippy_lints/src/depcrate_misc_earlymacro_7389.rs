// Generated macro for macro_7389 (macro)
macro_rules! Depcrate_misc_earlymacro_7389 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7389"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if literal suffixes are not separated by an"] # [doc = " underscore."] # [doc = " To enforce unseparated literal suffix style,"] # [doc = " see the `separated_literal_suffix` lint."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Suffix style should be consistent."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let _ ="] # [doc = " 123832i32"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let _ ="] # [doc = " 123832_i32"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNSEPARATED_LITERAL_SUFFIX , restriction , "literals whose suffix is not separated by an underscore" }
};
}
