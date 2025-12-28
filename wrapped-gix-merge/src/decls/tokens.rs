macro_rules! tokens {
    () => {
        pub fn tokens (input : & [u8]) -> imara_diff :: sources :: ByteLines < '_ > { imara_diff :: sources :: byte_lines (input) }
    };
}

tokens!()