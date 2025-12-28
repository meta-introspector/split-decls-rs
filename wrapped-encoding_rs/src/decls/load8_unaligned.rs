macro_rules! load8_unaligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an unaligned read of 16 bytes"] # [inline (always)] pub unsafe fn load8_unaligned (ptr : * const u16) -> u16x8 { let mut simd = :: core :: mem :: MaybeUninit :: < u16x8 > :: uninit () ; :: core :: ptr :: copy_nonoverlapping (ptr as * const u8 , simd . as_mut_ptr () as * mut u8 , 16) ; simd . assume_init () }
    };
}

load8_unaligned!();