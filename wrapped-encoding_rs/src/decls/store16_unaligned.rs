macro_rules! store16_unaligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an unaligned store of 16 bytes"] # [inline (always)] pub unsafe fn store16_unaligned (ptr : * mut u8 , s : u8x16) { :: core :: ptr :: copy_nonoverlapping (& s as * const u8x16 as * const u8 , ptr , 16) ; }
    };
}

store16_unaligned!();