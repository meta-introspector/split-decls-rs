macro_rules! dec_digit {
    () => {
        const fn dec_digit (c : u8) -> Option < u8 > { match c { b'0' ..= b'9' => Some (c - b'0') , _ => None , } }
    };
}

dec_digit!()