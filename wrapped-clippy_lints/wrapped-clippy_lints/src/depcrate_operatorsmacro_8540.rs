// Generated macro for macro_8540 (macro)
macro_rules! Depcrate_operatorsmacro_8540 {
() => {
// Module: crate::operators
// Provides: {"macro_8540"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for conversions to owned values just for the sake"] # [doc = " of a comparison."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The comparison can operate on a reference, so creating"] # [doc = " an owned value effectively throws it away directly afterwards, which is"] # [doc = " needlessly consuming code and heap space."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = \"foo\";"] # [doc = " # let y = String::from(\"foo\");"] # [doc = " if x.to_owned() == y {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = \"foo\";"] # [doc = " # let y = String::from(\"foo\");"] # [doc = " if x == y {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CMP_OWNED , perf , "creating owned instances for comparing with others, e.g., `x == \"foo\".to_string()`" }
};
}
