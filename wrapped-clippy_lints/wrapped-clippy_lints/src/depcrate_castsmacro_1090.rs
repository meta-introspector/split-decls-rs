// Generated macro for macro_1090 (macro)
macro_rules! Depcrate_castsmacro_1090 {
() => {
// Module: crate::casts
// Provides: {"macro_1090"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts from an enum type to an integral type that will definitely truncate the"] # [doc = " value."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The resulting integral value will not match the value of the variant it came from."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum E { X = 256 };"] # [doc = " let _ = E::X as u8;"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub CAST_ENUM_TRUNCATION , suspicious , "casts from an enum type to an integral type that will truncate the value" }
};
}
