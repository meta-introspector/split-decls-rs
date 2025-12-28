macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl AsRef < BStr > for Path < '_ > { fn as_ref (& self) -> & BStr { self . value . as_ref () } }
    };
}

impl_44!()