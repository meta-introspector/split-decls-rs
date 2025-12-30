// Generated macro for HexTransliterator (struct)
macro_rules! Depcrate_transliterate_transliterator_hardcodedHexTransliterator {
() => {
// Module: crate::transliterate::transliterator::hardcoded
// Provides: {"HexTransliterator"}
// Dependencies: {}
# [doc = " A transliterator that replaces every character with its `case`-case hexadecimal representation,"] # [doc = " 0-padded to `min_length`, and surrounded by `prefix` and `suffix`."] # [derive (Debug)] pub (super) struct HexTransliterator { prefix : & 'static str , suffix : & 'static str , min_length : u8 , case : Case , }
};
}
