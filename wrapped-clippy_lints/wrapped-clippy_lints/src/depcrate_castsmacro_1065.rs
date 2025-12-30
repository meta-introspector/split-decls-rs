// Generated macro for macro_1065 (macro)
macro_rules! Depcrate_castsmacro_1065 {
() => {
// Module: crate::casts
// Provides: {"macro_1065"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts from a signed to an unsigned numeric"] # [doc = " type. In this case, negative values wrap around to large positive values,"] # [doc = " which can be quite surprising in practice. However, since the cast works as"] # [doc = " defined, this lint is `Allow` by default."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Possibly surprising results. You can activate this lint"] # [doc = " as a one-time check to see where numeric wrapping can arise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let y: i8 = -1;"] # [doc = " y as u64; // will return 18446744073709551615"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CAST_SIGN_LOSS , pedantic , "casts from signed types to unsigned types, e.g., `x as u32` where `x: i32`" }
};
}
