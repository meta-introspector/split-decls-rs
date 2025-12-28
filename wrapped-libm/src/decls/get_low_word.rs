macro_rules! get_low_word {
    () => {
        # [inline] fn get_low_word (x : f64) -> u32 { x . to_bits () as u32 }
    };
}

get_low_word!()