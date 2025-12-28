macro_rules! utf16_char {
    () => {
        fn utf16_char (c : char) -> u16 { let buf = & mut [0u16 ; 2] ; let buf = c . encode_utf16 (buf) ; assert ! (buf . len () == 1) ; buf [0] }
    };
}

utf16_char!()