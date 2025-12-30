// Generated macro for impl_453 (impl)
macro_rules! Depcrate_harfbuzzimpl_453 {
() => {
// Module: crate::harfbuzz
// Provides: {"impl_453"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl GeneralCategoryFunc for CodePointMapDataBorrowed < '_ , GeneralCategory > { fn general_category (& self , ch : char) -> harfbuzz_traits :: GeneralCategory { self . get (ch) . into () } }
};
}
