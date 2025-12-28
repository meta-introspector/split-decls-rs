macro_rules! deps {
    () => {
        Xoshiro256PlusPlus!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl RngCore for Xoshiro256PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { let val = self . next_u64 () ; (val >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let res = self . s [0] . wrapping_add (self . s [3]) . rotate_left (23) . wrapping_add (self . s [0]) ; let t = self . s [1] << 17 ; self . s [2] ^= self . s [0] ; self . s [3] ^= self . s [1] ; self . s [1] ^= self . s [2] ; self . s [0] ^= self . s [3] ; self . s [2] ^= t ; self . s [3] = self . s [3] . rotate_left (45) ; res } # [inline] fn fill_bytes (& mut self , dst : & mut [u8]) { le :: fill_bytes_via_next (self , dst) } }
    };
}

impl_241!();