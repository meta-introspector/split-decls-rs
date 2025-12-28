macro_rules! calculate_hash {
    () => {
        fn calculate_hash < T : std :: hash :: Hash > (t : & T) -> u64 { use std :: hash :: Hasher ; let mut s = std :: collections :: hash_map :: DefaultHasher :: new () ; t . hash (& mut s) ; s . finish () }
    };
}

calculate_hash!();