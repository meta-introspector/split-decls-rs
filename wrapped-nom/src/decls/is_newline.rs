macro_rules! is_newline {
    () => {
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_newline`")] pub fn is_newline (chr : u8) -> bool { chr == b'\n' }
    };
}

is_newline!()