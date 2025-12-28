macro_rules! deps {
    () => {
        AsCCharPtr!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl AsCCharPtr for [u8] { fn as_c_char_ptr (& self) -> * const c_char { self . as_ptr () . cast () } }
    };
}

impl_198!()