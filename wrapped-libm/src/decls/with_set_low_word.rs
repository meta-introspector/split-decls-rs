macro_rules! with_set_low_word {
    () => {
        # [inline] fn with_set_low_word (f : f64 , lo : u32) -> f64 { let mut tmp = f . to_bits () ; tmp &= 0xffffffff_00000000 ; tmp |= lo as u64 ; f64 :: from_bits (tmp) }
    };
}

with_set_low_word!();