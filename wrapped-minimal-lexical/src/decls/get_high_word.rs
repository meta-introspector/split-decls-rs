macro_rules! get_high_word {
    () => {
        # [inline] fn get_high_word (x : f64) -> u32 { (x . to_bits () >> 32) as u32 }
    };
}

get_high_word!()