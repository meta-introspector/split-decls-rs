macro_rules! is_space {
    () => {
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_space`")] pub fn is_space (chr : u8) -> bool { chr == b' ' || chr == b'\t' }
    };
}

is_space!();