macro_rules! ascii_to_ascii_simd_stride {
    () => {
        # [allow (unused_macros)] macro_rules ! ascii_to_ascii_simd_stride { ($ name : ident , $ load : ident , $ store : ident) => { # [doc = " Safety: src and dst must be valid for 16 bytes of read/write according to"] # [doc = " the $load/$store fn, which may allow for unaligned reads/writes or require"] # [doc = " alignment to either 16x8 or u8x16."] # [inline (always)] pub unsafe fn $ name (src : * const u8 , dst : * mut u8) -> bool { let simd = $ load (src) ; if ! simd_is_ascii (simd) { return false ; } $ store (dst , simd) ; true } } ; }
    };
}

ascii_to_ascii_simd_stride!();