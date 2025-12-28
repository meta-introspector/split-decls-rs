macro_rules! deps {
    () => {
        Xoshiro128PlusPlus!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl RngCore for Xoshiro128PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { let res = self . s [0] . wrapping_add (self . s [3]) . rotate_left (7) . wrapping_add (self . s [0]) ; let t = self . s [1] << 9 ; self . s [2] ^= self . s [0] ; self . s [3] ^= self . s [1] ; self . s [1] ^= self . s [2] ; self . s [0] ^= self . s [3] ; self . s [2] ^= t ; self . s [3] = self . s [3] . rotate_left (11) ; res } # [inline] fn next_u64 (& mut self) -> u64 { le :: next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dst : & mut [u8]) { le :: fill_bytes_via_next (self , dst) } }
    };
}

impl_236!()