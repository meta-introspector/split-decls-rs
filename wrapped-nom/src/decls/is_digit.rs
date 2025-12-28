macro_rules! is_digit {
    () => {
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_dec_digit`")] pub fn is_digit (chr : u8) -> bool { matches ! (chr , 0x30 ..= 0x39) }
    };
}

is_digit!()