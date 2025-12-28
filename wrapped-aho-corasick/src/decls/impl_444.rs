macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < 'h , H : ? Sized + AsRef < [u8] > > From < & 'h H > for Input < 'h > { # [inline] fn from (haystack : & 'h H) -> Input < 'h > { Input :: new (haystack) } }
    };
}

impl_444!();