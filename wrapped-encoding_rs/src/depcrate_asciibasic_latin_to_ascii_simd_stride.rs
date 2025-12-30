// Generated macro for basic_latin_to_ascii_simd_stride (macro)
macro_rules! Depcrate_asciibasic_latin_to_ascii_simd_stride {
() => {
// Module: crate::ascii
// Provides: {"basic_latin_to_ascii_simd_stride"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! basic_latin_to_ascii_simd_stride { ($ name : ident , $ load : ident , $ store : ident) => { # [doc = " Safety: src and dst must be valid for 32/16 bytes of read/write according to"] # [doc = " the $load/$store fn, which may allow for unaligned reads/writes or require"] # [doc = " alignment to either 16x8 or u8x16."] # [inline (always)] pub unsafe fn $ name (src : * const u16 , dst : * mut u8) -> bool { let first = $ load (src) ; let second = $ load (src . add (8)) ; if simd_is_basic_latin (first | second) { $ store (dst , simd_pack (first , second)) ; true } else { false } } } ; }
};
}
