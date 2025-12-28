macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl AsRef < Self > for TokenStream { fn as_ref (& self) -> & Self { self } }
    };
}

impl_139!()