macro_rules! is_hex_digit {
    () => {
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_hex_digit`")] pub fn is_hex_digit (chr : u8) -> bool { matches ! (chr , 0x30 ..= 0x39 | 0x41 ..= 0x46 | 0x61 ..= 0x66) }
    };
}

is_hex_digit!()