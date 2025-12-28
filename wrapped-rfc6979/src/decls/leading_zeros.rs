macro_rules! leading_zeros {
    () => {
        # [doc = " Count the number of leading zeros in constant-time."] # [inline] pub (crate) fn leading_zeros (n : & [u8]) -> u32 { n [0] . leading_zeros () }
    };
}

leading_zeros!();