macro_rules! deps {
    () => {
        SharedSeed!();
        FoldHasher!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'a > FoldHasher < 'a > { # [doc = " Initializes this [`FoldHasher`] with the given per-hasher seed and"] # [doc = " [`SharedSeed`]."] # [inline] pub const fn with_seed (per_hasher_seed : u64 , shared_seed : & 'a SharedSeed) -> FoldHasher < 'a > { FoldHasher { accumulator : per_hasher_seed , sponge : 0 , sponge_len : 0 , seeds : & shared_seed . seeds , } } # [inline (always)] fn write_num < T : Into < u128 > > (& mut self , x : T) { let bits : usize = 8 * core :: mem :: size_of :: < T > () ; if self . sponge_len as usize + bits > 128 { let lo = self . sponge as u64 ; let hi = (self . sponge >> 64) as u64 ; self . accumulator = folded_multiply (lo ^ self . accumulator , hi ^ self . seeds [0]) ; self . sponge = x . into () ; self . sponge_len = bits as u8 ; } else { self . sponge |= x . into () << self . sponge_len ; self . sponge_len += bits as u8 ; } } }
    };
}

impl_1!();