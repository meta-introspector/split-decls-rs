macro_rules! count_multibyte_integer_size {
    () => {
        # [doc = " Count the number of bytes used by a multibyte integer."] fn count_multibyte_integer_size (data : & [u8]) -> usize { for (i , & byte) in data . iter () . enumerate () { if (byte & 0x80) == 0 { return i + 1 ; } } data . len () }
    };
}

count_multibyte_integer_size!();