// Generated macro for impl_456 (impl)
macro_rules! Depcrate_harfbuzzimpl_456 {
() => {
// Module: crate::harfbuzz
// Provides: {"impl_456"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl MirroringFunc for CodePointMapDataBorrowed < '_ , BidiMirroringGlyph > { fn mirroring (& self , ch : char) -> char { self . get (ch) . mirroring_glyph . unwrap_or (ch) } }
};
}
