macro_rules! deps {
    () => {
        TimeBuf!();
        Time!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Time { # [doc = " Serialize this instance into `buf`, exactly as it would appear in the header of a Git commit,"] # [doc = " and return `buf` as `&str` for easy consumption."] pub fn to_str < 'a > (& self , buf : & 'a mut TimeBuf) -> & 'a str { buf . clear () ; self . write_to (buf) . expect ("write to memory of just the right size cannot fail") ; buf . as_str () } }
    };
}

impl_27!();