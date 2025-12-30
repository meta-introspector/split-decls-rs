// Generated macro for macro_7388 (macro)
macro_rules! Depcrate_misc_earlymacro_7388 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7388"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns on hexadecimal literals with mixed-case letter"] # [doc = " digits."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It looks confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let _ ="] # [doc = " 0x1a9BAcD"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let _ ="] # [doc = " 0x1A9BACD"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MIXED_CASE_HEX_LITERALS , style , "hex literals whose letter digits are not consistently upper- or lowercased" }
};
}
