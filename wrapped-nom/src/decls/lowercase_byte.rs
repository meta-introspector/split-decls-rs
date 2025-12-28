macro_rules! lowercase_byte {
    () => {
        fn lowercase_byte (c : u8) -> u8 { match c { b'A' ..= b'Z' => c - b'A' + b'a' , _ => c , } }
    };
}

lowercase_byte!();