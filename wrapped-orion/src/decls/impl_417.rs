macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl FieldElement { pub fn new (value : u32) -> Self { debug_assert ! ((0 .. KYBER_Q) . contains (& value)) ; Self (value) } pub fn zero () -> Self { Self (0) } # [doc = " FIPS-203, def. 4.7."] # [doc = " Mapping of Z_q => Z_{2^{d}}."] # [doc = ""] # [doc = " This is a Rust port of:"] # [doc = " https://github.com/FiloSottile/mlkem768"] pub fn compress (& self , d : u8) -> u32 { debug_assert ! ((1 ..= 11) . contains (& d)) ; const MUL : u64 = 5039 ; const SHIFT : u64 = 24 ; let div : u32 = self . 0 << d ; let mut quo : u32 = ((u64 :: from (div) * MUL) >> SHIFT) as u32 ; let rem : u32 = div - (quo * KYBER_Q) ; quo += ((KYBER_Q / 2) . overflowing_sub (rem) . 0 >> 31) & 1 ; quo += ((KYBER_Q + KYBER_Q / 2 - rem) >> 31) & 1 ; let mask : u32 = (1 << d as u32) - 1 ; ((quo & mask) as u16) as u32 } # [doc = " FIPS-203, def. 4.8."] # [doc = " Mapping of Z_{2^{d}} => Z_q."] # [doc = ""] # [doc = " This is a Rust port of:"] # [doc = " https://github.com/FiloSottile/mlkem768"] pub fn decompress (y : u32 , d : u8) -> Self { debug_assert ! ((1 ..= 11) . contains (& d)) ; let div : u32 = y * KYBER_Q ; let mut quo : u32 = div >> d as u32 ; quo += (div >> (d as u32 - 1)) & 1 ; debug_assert ! (quo < KYBER_Q) ; FieldElement (quo) } # [cfg (all (test , feature = "safe_api"))] pub fn random () -> Self { use rand :: prelude :: * ; Self (rand :: rng () . random_range (0 .. KYBER_Q)) } }
    };
}

impl_417!();