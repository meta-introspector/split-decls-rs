macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl AsRef < [u8] > for TokenStream { fn as_ref (& self) -> & [u8] { self . 0 . as_ref () } }
    };
}

impl_140!();