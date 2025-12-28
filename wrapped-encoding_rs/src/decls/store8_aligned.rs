macro_rules! store8_aligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an aligned-for-u16x8 store of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn store8_aligned (ptr : * mut u16 , s : u16x8) { * (ptr as * mut u16x8) = s ; }
    };
}

store8_aligned!()