// Generated macro for macro_3419 (macro)
macro_rules! Depcrate_literal_representationmacro_3419 {
() => {
// Module: crate::literal_representation
// Provides: {"macro_3419"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if an integral or floating-point constant is"] # [doc = " grouped inconsistently with underscores."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readers may incorrectly interpret inconsistently"] # [doc = " grouped digits."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let _: u64 ="] # [doc = " 618_64_9189_73_511"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let _: u64 ="] # [doc = " 61_864_918_973_511"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INCONSISTENT_DIGIT_GROUPING , style , "integer literals with digits grouped inconsistently" }
};
}
