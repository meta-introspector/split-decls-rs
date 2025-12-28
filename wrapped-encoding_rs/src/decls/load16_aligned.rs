macro_rules! load16_aligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an aligned-for-u8x16 read of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn load16_aligned (ptr : * const u8) -> u8x16 { * (ptr as * const u8x16) }
    };
}

load16_aligned!();