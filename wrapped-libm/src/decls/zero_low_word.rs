macro_rules! zero_low_word {
    () => {
        fn zero_low_word (x : f64) -> f64 { f64 :: from_bits (f64 :: to_bits (x) & 0xFFFF_FFFF_0000_0000) }
    };
}

zero_low_word!()