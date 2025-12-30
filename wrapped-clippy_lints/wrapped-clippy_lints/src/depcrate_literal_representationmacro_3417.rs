// Generated macro for macro_3417 (macro)
macro_rules! Depcrate_literal_representationmacro_3417 {
() => {
// Module: crate::literal_representation
// Provides: {"macro_3417"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if a long integral or floating-point constant does"] # [doc = " not contain underscores."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Reading long numbers is difficult without separators."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let _: u64 ="] # [doc = " 61864918973511"] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let _: u64 ="] # [doc = " 61_864_918_973_511"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNREADABLE_LITERAL , pedantic , "long literal without underscores" }
};
}
