macro_rules! is_oct_digit {
    () => {
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_oct_digit`")] pub fn is_oct_digit (chr : u8) -> bool { matches ! (chr , 0x30 ..= 0x37) }
    };
}

is_oct_digit!()