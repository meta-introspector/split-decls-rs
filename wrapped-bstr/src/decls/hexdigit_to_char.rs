macro_rules! hexdigit_to_char {
    () => {
        # [doc = " Convert the given hexadecimal digit to its corresponding codepoint."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics when `digit > 15`."] fn hexdigit_to_char (digit : u8) -> char { char :: from_digit (u32 :: from (digit) , 16) . unwrap () . to_ascii_uppercase () }
    };
}

hexdigit_to_char!();