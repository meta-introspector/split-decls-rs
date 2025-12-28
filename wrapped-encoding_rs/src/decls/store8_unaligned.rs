macro_rules! store8_unaligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an unaligned store of 16 bytes"] # [inline (always)] pub unsafe fn store8_unaligned (ptr : * mut u16 , s : u16x8) { :: core :: ptr :: copy_nonoverlapping (& s as * const u16x8 as * const u8 , ptr as * mut u8 , 16) ; }
    };
}

store8_unaligned!()