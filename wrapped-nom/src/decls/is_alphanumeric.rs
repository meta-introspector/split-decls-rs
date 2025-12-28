macro_rules! deps {
    () => {
        AsChar!();
    };
}

macro_rules! is_alphanumeric {
    () => {
        deps!();
        # [inline] # [doc (hidden)] # [deprecated (since = "8.0.0" , note = "Replaced with `AsChar::is_alphanum`")] pub fn is_alphanumeric (chr : u8) -> bool { AsChar :: is_alphanum (chr) }
    };
}

is_alphanumeric!()