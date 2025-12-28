macro_rules! encode_unicode {
    () => {
        fn encode_unicode (s : Option < & str >) -> char { s . and_then (| s | u32 :: from_str_radix (s , 16) . ok () . and_then (char :: from_u32)) . unwrap_or (UNKNOWN_CHAR) }
    };
}

encode_unicode!()