macro_rules! deps {
    () => {
        AsCCharPtr!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl AsCCharPtr for str { fn as_c_char_ptr (& self) -> * const c_char { self . as_ptr () . cast () } }
    };
}

impl_197!()