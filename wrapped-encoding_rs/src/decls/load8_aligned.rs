macro_rules! load8_aligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an aligned-for-u16x8 read of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn load8_aligned (ptr : * const u16) -> u16x8 { * (ptr as * const u16x8) }
    };
}

load8_aligned!();