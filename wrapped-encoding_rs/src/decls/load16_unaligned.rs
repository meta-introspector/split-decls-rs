macro_rules! load16_unaligned {
    () => {
        # [doc = " Safety invariant: ptr must be valid for an unaligned read of 16 bytes"] # [inline (always)] pub unsafe fn load16_unaligned (ptr : * const u8) -> u8x16 { let mut simd = :: core :: mem :: MaybeUninit :: < u8x16 > :: uninit () ; :: core :: ptr :: copy_nonoverlapping (ptr , simd . as_mut_ptr () as * mut u8 , 16) ; simd . assume_init () }
    };
}

load16_unaligned!()