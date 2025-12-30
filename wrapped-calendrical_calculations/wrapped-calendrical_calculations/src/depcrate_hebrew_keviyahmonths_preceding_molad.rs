// Generated macro for months_preceding_molad (function)
macro_rules! Depcrate_hebrew_keviyahmonths_preceding_molad {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"months_preceding_molad"}
// Dependencies: {}
# [doc = " Calculate the number of months preceding the molad Tishrei for a given hebrew year (Tishrei is the first month)"] # [inline] fn months_preceding_molad (h_year : i32) -> i64 { (235 * (i64 :: from (h_year) - 1) + 1) . div_euclid (19) }
};
}
