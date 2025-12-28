macro_rules! ascii_to_basic_latin_simd_stride {
    () => {
        # [allow (unused_macros)] macro_rules ! ascii_to_basic_latin_simd_stride { ($ name : ident , $ load : ident , $ store : ident) => { # [doc = " Safety: src and dst must be valid for 16/32 bytes of read/write according to"] # [doc = " the $load/$store fn, which may allow for unaligned reads/writes or require"] # [doc = " alignment to either 16x8 or u8x16."] # [inline (always)] pub unsafe fn $ name (src : * const u8 , dst : * mut u16) -> bool { let simd = $ load (src) ; if ! simd_is_ascii (simd) { return false ; } let (first , second) = simd_unpack (simd) ; $ store (dst , first) ; $ store (dst . add (8) , second) ; true } } ; }
    };
}

ascii_to_basic_latin_simd_stride!()