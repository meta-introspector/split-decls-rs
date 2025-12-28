macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! first_byte_or_empty {
    () => {
        deps!();
        pub (crate) fn first_byte_or_empty (s : & str) -> Result < u8 , ParseError > { s . as_bytes () . first () . copied () . ok_or (perr (None , Empty)) }
    };
}

first_byte_or_empty!();