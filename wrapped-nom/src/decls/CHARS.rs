macro_rules! CHARS {
    () => {
        # [cfg (feature = "std")] static CHARS : & [u8] = b"0123456789abcdef" ;
    };
}

CHARS!();