macro_rules! store16_aligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an aligned-for-u8x16 store of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn store16_aligned (ptr : * mut u8 , s : u8x16) { * (ptr as * mut u8x16) = s ; }
    };
}

store16_aligned!();