macro_rules! hex_digit_value {
    () => {
        pub (crate) fn hex_digit_value (digit : u8) -> Option < u8 > { match digit { b'0' ..= b'9' => Some (digit - b'0') , b'a' ..= b'f' => Some (digit - b'a' + 10) , b'A' ..= b'F' => Some (digit - b'A' + 10) , _ => None , } }
    };
}

hex_digit_value!();