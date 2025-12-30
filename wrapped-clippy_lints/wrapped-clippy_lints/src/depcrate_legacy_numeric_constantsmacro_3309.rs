// Generated macro for macro_3309 (macro)
macro_rules! Depcrate_legacy_numeric_constantsmacro_3309 {
() => {
// Module: crate::legacy_numeric_constants
// Provides: {"macro_3309"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `<integer>::max_value()`, `std::<integer>::MAX`,"] # [doc = " `std::<float>::EPSILON`, etc."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " All of these have been superseded by the associated constants on their respective types,"] # [doc = " such as `i128::MAX`. These legacy items may be deprecated in a future version of rust."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " let eps = std::f32::EPSILON;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " let eps = f32::EPSILON;"] # [doc = " ```"] # [clippy :: version = "1.79.0"] pub LEGACY_NUMERIC_CONSTANTS , style , "checks for usage of legacy std numeric constants and methods" }
};
}
