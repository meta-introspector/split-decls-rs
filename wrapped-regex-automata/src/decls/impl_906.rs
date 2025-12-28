macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! impl_906 {
    () => {
        deps!();
        impl < 'h , H : ? Sized + AsRef < [u8] > > From < & 'h H > for Input < 'h > { fn from (haystack : & 'h H) -> Input < 'h > { Input :: new (haystack) } }
    };
}

impl_906!();