macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl AsRef < [u8] > for Path < '_ > { fn as_ref (& self) -> & [u8] { self . value . as_ref () } }
    };
}

impl_43!();