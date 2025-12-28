macro_rules! iso_2022_jp_ascii_valid_up_to {
    () => {
        pub fn iso_2022_jp_ascii_valid_up_to (bytes : & [u8]) -> usize { for (i , b_ref) in bytes . iter () . enumerate () { let b = * b_ref ; if b >= 0x80 || b == 0x1B || b == 0x0E || b == 0x0F { return i ; } } bytes . len () }
    };
}

iso_2022_jp_ascii_valid_up_to!()