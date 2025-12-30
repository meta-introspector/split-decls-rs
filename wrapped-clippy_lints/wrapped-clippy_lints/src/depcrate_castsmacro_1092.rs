// Generated macro for macro_1092 (macro)
macro_rules! Depcrate_castsmacro_1092 {
() => {
// Module: crate::casts
// Provides: {"macro_1092"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts from an enum tuple constructor to an integer."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The cast is easily confused with casting a c-like enum value to an integer."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum E { X(i32) };"] # [doc = " let _ = E::X as usize;"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub CAST_ENUM_CONSTRUCTOR , suspicious , "casts from an enum tuple constructor to an integer" }
};
}
