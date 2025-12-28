macro_rules! char_to_hexdigit {
    () => {
        # [doc = " Convert the given codepoint to its corresponding hexadecimal digit."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if `ch` is not in `[0-9A-Fa-f]`."] # [cfg (feature = "alloc")] fn char_to_hexdigit (ch : char) -> u8 { u8 :: try_from (ch . to_digit (16) . unwrap ()) . unwrap () }
    };
}

char_to_hexdigit!()