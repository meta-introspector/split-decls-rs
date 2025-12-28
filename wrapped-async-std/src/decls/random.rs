macro_rules! random {
    () => {
        # [doc = " Generates a random number in `0..n`."] # [cfg (feature = "unstable")] pub fn random (n : u32) -> u32 { use std :: cell :: Cell ; use std :: num :: Wrapping ; thread_local ! { static RNG : Cell < Wrapping < u32 >> = { let mut x = 0i32 ; let r = & mut x ; let addr = r as * mut i32 as usize ; Cell :: new (Wrapping (addr as u32)) } } RNG . with (| rng | { let mut x = rng . get () ; x ^= x << 13 ; x ^= x >> 17 ; x ^= x << 5 ; rng . set (x) ; ((u64 :: from (x . 0)) . wrapping_mul (u64 :: from (n)) >> 32) as u32 }) }
    };
}

random!();