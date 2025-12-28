macro_rules! ascii_valid_up_to {
    () => {
        pub fn ascii_valid_up_to (bytes : & [u8]) -> usize { match validate_ascii (bytes) { None => bytes . len () , Some ((_ , num_valid)) => num_valid , } }
    };
}

ascii_valid_up_to!();