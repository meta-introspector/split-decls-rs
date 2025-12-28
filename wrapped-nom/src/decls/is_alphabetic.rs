macro_rules! is_alphabetic {
    () => {
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_alpha`")] pub fn is_alphabetic (chr : u8) -> bool { matches ! (chr , 0x41 ..= 0x5A | 0x61 ..= 0x7A) }
    };
}

is_alphabetic!()